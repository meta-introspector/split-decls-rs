// Generated macro for panic (function)
macro_rules! Depcrate_exportpanic {
() => {
// Module: crate::export
// Provides: {"panic"}
// Dependencies: {}
# [cfg (not (feature = "unstable-test"))] # [inline (always)] pub fn panic () -> ! { extern "Rust" { fn _defmt_panic () -> ! ; } unsafe { _defmt_panic () } }
};
}
