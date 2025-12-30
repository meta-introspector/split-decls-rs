// Generated macro for impl_256 (impl)
macro_rules! Depcrate_rsaimpl_256 {
() => {
// Module: crate::rsa
// Provides: {"impl_256"}
// Dependencies: {}
impl Drop for PrivateKey { fn drop (& mut self) { unsafe { bssl_sys :: RSA_free (self . 0) } } }
};
}
