// Generated macro for impl_109 (impl)
macro_rules! Depcrate_signingimpl_109 {
() => {
// Module: crate::signing
// Provides: {"impl_109"}
// Dependencies: {}
# [doc = " Constant-time comparison"] impl < C > Eq for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { }
};
}
