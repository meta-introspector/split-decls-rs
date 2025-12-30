// Generated macro for impl_1730 (impl)
macro_rules! Depcrate_tlsimpl_1730 {
() => {
// Module: crate::tls
// Provides: {"impl_1730"}
// Dependencies: {}
impl Drop for Handshake { fn drop (& mut self) { unsafe { SSL_free (self . as_mut_ptr ()) } } }
};
}
