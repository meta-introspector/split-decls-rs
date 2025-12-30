// Generated macro for impl_49 (impl)
macro_rules! Depcrate_ecdsaimpl_49 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_49"}
// Dependencies: {}
impl TryFrom < & [u8] > for Signature { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Signature > { Signature :: from_slice (bytes) } }
};
}
