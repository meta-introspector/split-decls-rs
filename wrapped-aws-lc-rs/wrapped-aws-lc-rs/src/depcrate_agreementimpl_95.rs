// Generated macro for impl_95 (impl)
macro_rules! Depcrate_agreementimpl_95 {
() => {
// Module: crate::agreement
// Provides: {"impl_95"}
// Dependencies: {}
impl KeyInner { # [inline] fn algorithm (& self) -> & 'static Algorithm { match self { KeyInner :: ECDH_P256 (..) => & ECDH_P256 , KeyInner :: ECDH_P384 (..) => & ECDH_P384 , KeyInner :: ECDH_P521 (..) => & ECDH_P521 , KeyInner :: X25519 (..) => & X25519 , } } fn get_evp_pkey (& self) -> & LcPtr < EVP_PKEY > { match self { KeyInner :: ECDH_P256 (evp_pkey) | KeyInner :: ECDH_P384 (evp_pkey) | KeyInner :: ECDH_P521 (evp_pkey) | KeyInner :: X25519 (evp_pkey) => evp_pkey , } } }
};
}
