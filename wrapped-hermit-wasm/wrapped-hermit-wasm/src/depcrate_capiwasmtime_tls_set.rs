// Generated macro for wasmtime_tls_set (function)
macro_rules! Depcrate_capiwasmtime_tls_set {
() => {
// Module: crate::capi
// Provides: {"wasmtime_tls_set"}
// Dependencies: {}
# [doc = ""] # [doc = " This value should be returned when later calling `wasmtime_tls_get`."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_tls_set (ptr : * mut u8) { unsafe { TLS . get () . write (ptr) ; } }
};
}
