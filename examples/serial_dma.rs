//! Write a string to the serial port every half second using DMA.
//!
//! Note: This example is for the STM32F746


#![deny(unsafe_code)]
#![deny(warnings)]

#![no_main]
#![no_std]


extern crate panic_halt;


use cortex_m_rt::entry;
use stm32f7xx_hal::{
    prelude::*,
    device,
    dma::DMA,
    serial::{
        self,
        Serial,
    },
    delay::Delay,
};


#[entry]
fn main() -> ! {
    let p = device::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    let     dma    = DMA::new(p.DMA1);
    let mut stream = dma.streams.stream3;
    let     dma    = dma.handle.enable(&mut rcc);


    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let gpiod = p.GPIOD.split();

    let tx = gpiod.pd8.into_alternate_af7();
    let rx = gpiod.pd9.into_alternate_af7();

    let serial = Serial::new(
        p.USART3,
        (tx, rx),
        clocks,
        serial::Config {
            baud_rate: 115_200.bps(),
            oversampling: serial::Oversampling::By16,
        },
    );
    let (mut tx, _) = serial.split();

    let mut hello = b"Hello, I'm a STM32F7xx!\r\n".as_ref();
    loop {
        let mut transfer = tx.write_all(hello, &dma, stream);
        transfer.start(&dma);

        let res = transfer.wait(&dma)
            .unwrap();
        hello  = res.source;
        tx     = res.target;
        stream = res.stream;

        delay.delay_ms(500u16);
    }
}
