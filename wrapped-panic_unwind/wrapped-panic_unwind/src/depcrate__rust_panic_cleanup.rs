// Generated macro for __rust_panic_cleanup (function)
macro_rules! Depcrate__rust_panic_cleanup {
() => {
// Module: crate
// Provides: {"__rust_panic_cleanup"}
// Dependencies: {}
# [rustc_std_internal_symbol] # [allow (improper_ctypes_definitions)] pub unsafe extern "C" fn __rust_panic_cleanup (payload : * mut u8) -> * mut (dyn Any + Send + 'static) { unsafe { Box :: into_raw (imp :: cleanup (payload)) } }
};
}
