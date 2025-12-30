// Generated macro for impl_56 (impl)
macro_rules! Depcrate_schnorrimpl_56 {
() => {
// Module: crate::schnorr
// Provides: {"impl_56"}
// Dependencies: {}
impl TryFrom < SignatureBytes > for Signature { type Error = Error ; fn try_from (signature : SignatureBytes) -> Result < Signature > { Signature :: from_bytes (& signature) } }
};
}
