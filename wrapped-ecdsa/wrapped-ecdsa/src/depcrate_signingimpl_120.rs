// Generated macro for impl_120 (impl)
macro_rules! Depcrate_signingimpl_120 {
() => {
// Module: crate::signing
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > KeypairRef for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type VerifyingKey = VerifyingKey < C > ; }
};
}
