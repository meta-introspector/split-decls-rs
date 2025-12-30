// Generated macro for _panic (function)
macro_rules! Depcrate_panic {
() => {
// Module: crate
// Provides: {"_panic"}
// Dependencies: {}
# [cfg_attr (not (test) , panic_handler)] # [expect (clippy :: empty_loop)] fn _panic (_ : & core :: panic :: PanicInfo) -> ! { loop { } }
};
}
