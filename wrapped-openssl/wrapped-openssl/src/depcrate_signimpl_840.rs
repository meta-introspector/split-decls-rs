// Generated macro for impl_840 (impl)
macro_rules! Depcrate_signimpl_840 {
() => {
// Module: crate::sign
// Provides: {"impl_840"}
// Dependencies: {}
impl Drop for Verifier < '_ > { fn drop (& mut self) { unsafe { EVP_MD_CTX_free (self . md_ctx) ; } } }
};
}
