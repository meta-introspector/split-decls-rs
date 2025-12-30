// Generated macro for impl_118 (impl)
macro_rules! Depcrate_signingimpl_118 {
() => {
// Module: crate::signing
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > From < SigningKey < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (signing_key : SigningKey < C >) -> VerifyingKey < C > { signing_key . verifying_key } }
};
}
