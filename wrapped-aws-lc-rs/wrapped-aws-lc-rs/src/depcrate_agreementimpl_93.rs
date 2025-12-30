// Generated macro for impl_93 (impl)
macro_rules! Depcrate_agreementimpl_93 {
() => {
// Module: crate::agreement
// Provides: {"impl_93"}
// Dependencies: {}
impl Clone for KeyInner { fn clone (& self) -> KeyInner { match self { KeyInner :: ECDH_P256 (evp_pkey) => KeyInner :: ECDH_P256 (evp_pkey . clone ()) , KeyInner :: ECDH_P384 (evp_pkey) => KeyInner :: ECDH_P384 (evp_pkey . clone ()) , KeyInner :: ECDH_P521 (evp_pkey) => KeyInner :: ECDH_P521 (evp_pkey . clone ()) , KeyInner :: X25519 (evp_pkey) => KeyInner :: X25519 (evp_pkey . clone ()) , } } }
};
}
