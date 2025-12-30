// Generated macro for impl_1725 (impl)
macro_rules! Depcrate_tlsimpl_1725 {
() => {
// Module: crate::tls
// Provides: {"impl_1725"}
// Dependencies: {}
impl Drop for Context { fn drop (& mut self) { unsafe { SSL_CTX_free (self . as_mut_ptr ()) } } }
};
}
