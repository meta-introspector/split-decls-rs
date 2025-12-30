// Generated macro for release (function)
macro_rules! Depcrate_exportrelease {
() => {
// Module: crate::export
// Provides: {"release"}
// Dependencies: {}
# [doc = " Only to be used by the defmt macros"] # [doc = " Safety: must follow an earlier call to acquire()"] # [cfg (not (feature = "unstable-test"))] # [inline (always)] pub unsafe fn release () { extern "Rust" { fn _defmt_release () ; } _defmt_release () }
};
}
