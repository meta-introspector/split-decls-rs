// Generated macro for __rust_start_panic (function)
macro_rules! Depcrate__rust_start_panic {
() => {
// Module: crate
// Provides: {"__rust_start_panic"}
// Dependencies: {}
# [rustc_std_internal_symbol] pub unsafe fn __rust_start_panic (_payload : & mut dyn PanicPayload) -> u32 { # [cfg (target_os = "android")] unsafe { android :: android_set_abort_message (_payload) ; } # [cfg (target_os = "zkvm")] unsafe { zkvm :: zkvm_set_abort_message (_payload) ; } unsafe extern "Rust" { # [rustc_std_internal_symbol] safe fn __rust_abort () -> !; } __rust_abort () }
};
}
