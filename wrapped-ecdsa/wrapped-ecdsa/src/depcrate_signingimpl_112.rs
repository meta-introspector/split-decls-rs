// Generated macro for impl_112 (impl)
macro_rules! Depcrate_signingimpl_112 {
() => {
// Module: crate::signing
// Provides: {"impl_112"}
// Dependencies: {}
impl < C > From < SecretKey < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (secret_key : SecretKey < C >) -> Self { Self :: from (& secret_key) } }
};
}
