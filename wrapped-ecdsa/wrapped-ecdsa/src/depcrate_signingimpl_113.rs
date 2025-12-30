// Generated macro for impl_113 (impl)
macro_rules! Depcrate_signingimpl_113 {
() => {
// Module: crate::signing
// Provides: {"impl_113"}
// Dependencies: {}
impl < C > From < & SecretKey < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_key : & SecretKey < C >) -> Self { secret_key . to_nonzero_scalar () . into () } }
};
}
