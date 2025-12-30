// Generated macro for impl_465 (impl)
macro_rules! Depcrate_verifierimpl_465 {
() => {
// Module: crate::verifier
// Provides: {"impl_465"}
// Dependencies: {}
impl rustls_client_cert_verifier { # [doc = " Free a `rustls_client_cert_verifier` previously returned from"] # [doc = " `rustls_client_cert_verifier_builder_build`. Calling with NULL is fine. Must not be"] # [doc = " called twice with the same value."] # [no_mangle] pub extern "C" fn rustls_client_cert_verifier_free (verifier : * mut rustls_client_cert_verifier) { ffi_panic_boundary ! { free_box (verifier) ; } } }
};
}
