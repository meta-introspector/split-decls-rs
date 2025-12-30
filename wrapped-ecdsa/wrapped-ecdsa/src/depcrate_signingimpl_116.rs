// Generated macro for impl_116 (impl)
macro_rules! Depcrate_signingimpl_116 {
() => {
// Module: crate::signing
// Provides: {"impl_116"}
// Dependencies: {}
impl < C > TryFrom < & [u8] > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self > { Self :: from_slice (bytes) } }
};
}
