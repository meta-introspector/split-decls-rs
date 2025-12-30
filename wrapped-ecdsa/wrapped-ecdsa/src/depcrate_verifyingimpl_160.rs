// Generated macro for impl_160 (impl)
macro_rules! Depcrate_verifyingimpl_160 {
() => {
// Module: crate::verifying
// Provides: {"impl_160"}
// Dependencies: {}
impl < C > From < & PublicKey < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (public_key : & PublicKey < C >) -> VerifyingKey < C > { (* public_key) . into () } }
};
}
