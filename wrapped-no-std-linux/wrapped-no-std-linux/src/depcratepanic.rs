// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [inline (never)] # [panic_handler] fn panic (_info : & core :: panic :: PanicInfo < '_ >) -> ! { unsafe { libc :: puts ("panicked\0" as * const str as * const i8) ; libc :: abort () } }
};
}
