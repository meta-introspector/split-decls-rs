// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [cfg (all (not (feature = "std") , feature = "looping_panic_handler"))] # [panic_handler] fn panic (_info : & core :: panic :: PanicInfo) -> ! { loop { } }
};
}
