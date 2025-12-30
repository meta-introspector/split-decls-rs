// Generated macro for impl_58 (impl)
macro_rules! Depcrate_schnorrimpl_58 {
() => {
// Module: crate::schnorr
// Provides: {"impl_58"}
// Dependencies: {}
impl TryFrom < & [u8] > for Signature { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Signature > { Signature :: from_slice (bytes) } }
};
}
