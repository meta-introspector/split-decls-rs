// Generated macro for panic (function)
macro_rules! Depcratepanic {
() => {
// Module: crate
// Provides: {"panic"}
// Dependencies: {}
# [cfg (not (any (test , all (target_arch = "wasm32" , target_env = "p2"))))] # [panic_handler] fn panic (_info : & core :: panic :: PanicInfo) -> ! { unsafe extern "C" { fn panic_nonexistent () -> ! ; } unsafe { panic_nonexistent () } }
};
}
