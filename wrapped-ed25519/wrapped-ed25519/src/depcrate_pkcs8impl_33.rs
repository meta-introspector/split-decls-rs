// Generated macro for impl_33 (impl)
macro_rules! Depcrate_pkcs8impl_33 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_33"}
// Dependencies: {}
impl TryFrom < KeypairBytes > for PublicKeyBytes { type Error = spki :: Error ; fn try_from (keypair : KeypairBytes) -> spki :: Result < PublicKeyBytes > { PublicKeyBytes :: try_from (& keypair) } }
};
}
