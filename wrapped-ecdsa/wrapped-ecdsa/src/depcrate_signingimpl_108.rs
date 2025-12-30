// Generated macro for impl_108 (impl)
macro_rules! Depcrate_signingimpl_108 {
() => {
// Module: crate::signing
// Provides: {"impl_108"}
// Dependencies: {}
impl < C > Drop for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn drop (& mut self) { self . secret_scalar . zeroize () ; } }
};
}
