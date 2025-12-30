// Generated macro for impl_179 (impl)
macro_rules! Depcrate_hmacimpl_179 {
() => {
// Module: crate::hmac
// Provides: {"impl_179"}
// Dependencies: {}
impl < const N : usize , MD : digest :: Algorithm > Drop for Hmac < N , MD > { fn drop (& mut self) { unsafe { bssl_sys :: HMAC_CTX_cleanup (& mut self . ctx) } } }
};
}
