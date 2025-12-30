// Generated macro for impl_249 (impl)
macro_rules! Depcrate_rsaimpl_249 {
() => {
// Module: crate::rsa
// Provides: {"impl_249"}
// Dependencies: {}
impl Drop for PublicKey { fn drop (& mut self) { unsafe { bssl_sys :: RSA_free (self . 0) } } }
};
}
