// Generated macro for impl_32 (impl)
macro_rules! Depcrate_pkcs8impl_32 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_32"}
// Dependencies: {}
impl TryFrom < & [u8] > for PublicKeyBytes { type Error = spki :: Error ; fn try_from (der_bytes : & [u8]) -> spki :: Result < Self > { Self :: from_public_key_der (der_bytes) } }
};
}
