// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [inline (never)] # [panic_handler] fn panic (info : & core :: panic :: PanicInfo < '_ >) -> ! { macro_rules ! println { ($ ($ tt : tt) *) => { use core :: fmt :: Write as _ ; let _ = writeln ! (simio :: Console , $ ($ tt) *) ; } ; } println ! ("{info}") ; # [allow (clippy :: empty_loop)] loop { } }
};
}
