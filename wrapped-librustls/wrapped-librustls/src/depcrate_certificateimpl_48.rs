// Generated macro for impl_48 (impl)
macro_rules! Depcrate_certificateimpl_48 {
() => {
// Module: crate::certificate
// Provides: {"impl_48"}
// Dependencies: {}
impl rustls_root_cert_store { # [doc = " Free a rustls_root_cert_store previously returned from rustls_root_cert_store_builder_build."] # [doc = ""] # [doc = " Calling with NULL is fine. Must not be called twice with the same value."] # [no_mangle] pub extern "C" fn rustls_root_cert_store_free (store : * const rustls_root_cert_store) { ffi_panic_boundary ! { free_arc (store) ; } } }
};
}
