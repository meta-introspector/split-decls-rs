// Generated macro for impl_66 (impl)
macro_rules! Depcrate_public_keyimpl_66 {
() => {
// Module: crate::public_key
// Provides: {"impl_66"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl TryFrom < pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for PublicKey { type Error = pkcs8 :: spki :: Error ; fn try_from (spki : pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> pkcs8 :: spki :: Result < Self > { Self :: try_from (& spki) } }
};
}
