// Generated macro for impl_24 (impl)
macro_rules! Depcrate_pkcs8impl_24 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_24"}
// Dependencies: {}
impl TryFrom < & [u8] > for KeypairBytes { type Error = Error ; fn try_from (der_bytes : & [u8]) -> Result < Self > { Self :: from_pkcs8_der (der_bytes) } }
};
}
