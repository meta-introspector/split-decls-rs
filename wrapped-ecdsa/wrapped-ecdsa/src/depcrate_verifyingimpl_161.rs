// Generated macro for impl_161 (impl)
macro_rules! Depcrate_verifyingimpl_161 {
() => {
// Module: crate::verifying
// Provides: {"impl_161"}
// Dependencies: {}
impl < C > From < VerifyingKey < C > > for PublicKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (verifying_key : VerifyingKey < C >) -> PublicKey < C > { verifying_key . inner } }
};
}
