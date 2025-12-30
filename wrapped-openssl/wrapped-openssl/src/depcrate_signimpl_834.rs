// Generated macro for impl_834 (impl)
macro_rules! Depcrate_signimpl_834 {
() => {
// Module: crate::sign
// Provides: {"impl_834"}
// Dependencies: {}
impl Drop for Signer < '_ > { fn drop (& mut self) { unsafe { EVP_MD_CTX_free (self . md_ctx) ; } } }
};
}
