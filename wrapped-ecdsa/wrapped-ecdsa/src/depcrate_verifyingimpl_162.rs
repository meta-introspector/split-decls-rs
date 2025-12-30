// Generated macro for impl_162 (impl)
macro_rules! Depcrate_verifyingimpl_162 {
() => {
// Module: crate::verifying
// Provides: {"impl_162"}
// Dependencies: {}
impl < C > From < & VerifyingKey < C > > for PublicKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (verifying_key : & VerifyingKey < C >) -> PublicKey < C > { (* verifying_key) . into () } }
};
}
