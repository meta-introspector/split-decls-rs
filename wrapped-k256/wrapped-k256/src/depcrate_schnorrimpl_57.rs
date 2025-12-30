// Generated macro for impl_57 (impl)
macro_rules! Depcrate_schnorrimpl_57 {
() => {
// Module: crate::schnorr
// Provides: {"impl_57"}
// Dependencies: {}
impl TryFrom < & SignatureBytes > for Signature { type Error = Error ; fn try_from (signature : & SignatureBytes) -> Result < Signature > { Signature :: from_bytes (signature) } }
};
}
