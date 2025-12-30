// Generated macro for impl_159 (impl)
macro_rules! Depcrate_verifyingimpl_159 {
() => {
// Module: crate::verifying
// Provides: {"impl_159"}
// Dependencies: {}
impl < C > From < PublicKey < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (public_key : PublicKey < C >) -> VerifyingKey < C > { VerifyingKey { inner : public_key } } }
};
}
