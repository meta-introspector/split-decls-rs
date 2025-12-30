// Generated macro for impl_117 (impl)
macro_rules! Depcrate_signingimpl_117 {
() => {
// Module: crate::signing
// Provides: {"impl_117"}
// Dependencies: {}
impl < C > ZeroizeOnDrop for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { }
};
}
