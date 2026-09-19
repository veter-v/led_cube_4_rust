#![no_std]
#![no_main]

mod lighting_modes;

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut layer_pins = [
        pins.a0.into_output().downgrade(),
        pins.a1.into_output().downgrade(),
        pins.a2.into_output().downgrade(),
        pins.a3.into_output().downgrade(),
    ];

    let mut column_pins = [
        pins.a4.into_output().downgrade(),
        pins.a5.into_output().downgrade(),
        pins.d0.into_output().downgrade(),
        pins.d1.into_output().downgrade(),
        pins.d2.into_output().downgrade(),
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
        pins.d9.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d11.into_output().downgrade(),
        pins.d12.into_output().downgrade(),
        pins.d13.into_output().downgrade(),
    ];

    loop {
        // пока временный цикл с delay или как делать не надо!

        for pin in &mut layer_pins {
            pin.toggle();
        }
        delay_ms(2000);
    }
}
