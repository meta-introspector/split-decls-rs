// Generated macro for impl_169 (impl)
macro_rules! Depcrate_crypto_providerimpl_169 {
() => {
// Module: crate::crypto_provider
// Provides: {"impl_169"}
// Dependencies: {}
impl rustls_signing_key { # [doc = " Frees the `rustls_signing_key`. This is safe to call with a `NULL` argument, but"] # [doc = " must not be called twice with the same value."] # [no_mangle] pub extern "C" fn rustls_signing_key_free (signing_key : * mut rustls_signing_key) { ffi_panic_boundary ! { free_box (signing_key) ; } } }
};
}
