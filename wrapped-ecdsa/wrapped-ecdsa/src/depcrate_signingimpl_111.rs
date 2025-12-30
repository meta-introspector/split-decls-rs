// Generated macro for impl_111 (impl)
macro_rules! Depcrate_signingimpl_111 {
() => {
// Module: crate::signing
// Provides: {"impl_111"}
// Dependencies: {}
impl < C > From < NonZeroScalar < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_scalar : NonZeroScalar < C >) -> Self { # [cfg (feature = "algorithm")] let public_key = PublicKey :: from_secret_scalar (& secret_scalar) ; Self { secret_scalar , # [cfg (feature = "algorithm")] verifying_key : public_key . into () , } } }
};
}
