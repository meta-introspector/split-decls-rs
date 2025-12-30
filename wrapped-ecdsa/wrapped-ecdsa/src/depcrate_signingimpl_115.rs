// Generated macro for impl_115 (impl)
macro_rules! Depcrate_signingimpl_115 {
() => {
// Module: crate::signing
// Provides: {"impl_115"}
// Dependencies: {}
impl < C > From < & SigningKey < C > > for SecretKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_key : & SigningKey < C >) -> Self { secret_key . secret_scalar . into () } }
};
}
