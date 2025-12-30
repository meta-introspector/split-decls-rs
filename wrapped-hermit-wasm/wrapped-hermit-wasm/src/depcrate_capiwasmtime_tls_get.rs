// Generated macro for wasmtime_tls_get (function)
macro_rules! Depcrate_capiwasmtime_tls_get {
() => {
// Module: crate::capi
// Provides: {"wasmtime_tls_get"}
// Dependencies: {}
# [doc = " Wasmtime requires a single pointer's space of TLS to be used at runtime,"] # [doc = " and this function returns the current value of the TLS variable."] # [doc = ""] # [doc = " This value should default to `NULL`."] # [unsafe (no_mangle)] pub extern "C" fn wasmtime_tls_get () -> * mut u8 { unsafe { TLS . get () . read () } }
};
}
