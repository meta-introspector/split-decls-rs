// Generated macro for impl_47 (impl)
macro_rules! Depcrate_ecdsaimpl_47 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_47"}
// Dependencies: {}
impl TryFrom < SignatureBytes > for Signature { type Error = Error ; fn try_from (signature : SignatureBytes) -> Result < Signature > { Signature :: from_bytes (& signature) } }
};
}
