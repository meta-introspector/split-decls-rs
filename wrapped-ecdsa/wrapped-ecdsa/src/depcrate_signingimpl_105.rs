// Generated macro for impl_105 (impl)
macro_rules! Depcrate_signingimpl_105 {
() => {
// Module: crate::signing
// Provides: {"impl_105"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > AsRef < VerifyingKey < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn as_ref (& self) -> & VerifyingKey < C > { & self . verifying_key } }
};
}
