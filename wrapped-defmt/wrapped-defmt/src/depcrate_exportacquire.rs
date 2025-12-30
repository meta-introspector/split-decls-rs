// Generated macro for acquire (function)
macro_rules! Depcrate_exportacquire {
() => {
// Module: crate::export
// Provides: {"acquire"}
// Dependencies: {}
# [doc = " Only to be used by the defmt macros"] # [doc = " Safety: must be paired with a later call to release()"] # [cfg (not (feature = "unstable-test"))] # [inline (always)] pub unsafe fn acquire () { extern "Rust" { fn _defmt_acquire () ; } _defmt_acquire () }
};
}
