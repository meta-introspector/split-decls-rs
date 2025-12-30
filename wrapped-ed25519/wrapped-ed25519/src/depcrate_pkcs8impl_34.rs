// Generated macro for impl_34 (impl)
macro_rules! Depcrate_pkcs8impl_34 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_34"}
// Dependencies: {}
impl TryFrom < & KeypairBytes > for PublicKeyBytes { type Error = spki :: Error ; fn try_from (keypair : & KeypairBytes) -> spki :: Result < PublicKeyBytes > { keypair . public_key . ok_or (spki :: Error :: KeyMalformed) } }
};
}
