// Generated macro for impl_288 (impl)
macro_rules! Depcrate_hmacimpl_288 {
() => {
// Module: crate::hmac
// Provides: {"impl_288"}
// Dependencies: {}
impl Drop for LcHmacCtx { fn drop (& mut self) { unsafe { HMAC_CTX_cleanup (self . as_mut_ptr ()) } } }
};
}
