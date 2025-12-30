// Generated macro for impl_48 (impl)
macro_rules! Depcrate_ecdsaimpl_48 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_48"}
// Dependencies: {}
impl TryFrom < & SignatureBytes > for Signature { type Error = Error ; fn try_from (signature : & SignatureBytes) -> Result < Signature > { Signature :: from_bytes (signature) } }
};
}
