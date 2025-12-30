// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [inline (never)] # [panic_handler] fn panic (info : & core :: panic :: PanicInfo < '_ >) -> ! { avr_device :: interrupt :: disable () ; let dp = unsafe { arduino_hal :: Peripherals :: steal () } ; let pins = arduino_hal :: pins ! (dp) ; let mut serial = sim :: Usart (arduino_hal :: default_serial ! (dp , pins , 57600)) ; macro_rules ! println { ($ ($ tt : tt) *) => { { use core :: fmt :: Write as _ ; let _ = writeln ! (serial , $ ($ tt) *) ; } } ; } println ! ("{info}") ; sim :: exit (1) }
};
}
