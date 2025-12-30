// Generated macro for impl_204 (impl)
macro_rules! Depcrateimpl_204 {
() => {
// Module: crate
// Provides: {"impl_204"}
// Dependencies: {}
impl < C > TryFrom < & [u8] > for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { type Error = Error ; fn try_from (slice : & [u8]) -> Result < Self > { Self :: from_slice (slice) } }
};
}
