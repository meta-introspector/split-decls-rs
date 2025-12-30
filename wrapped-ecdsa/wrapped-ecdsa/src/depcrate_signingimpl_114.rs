// Generated macro for impl_114 (impl)
macro_rules! Depcrate_signingimpl_114 {
() => {
// Module: crate::signing
// Provides: {"impl_114"}
// Dependencies: {}
impl < C > From < SigningKey < C > > for SecretKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (key : SigningKey < C >) -> Self { key . secret_scalar . into () } }
};
}
