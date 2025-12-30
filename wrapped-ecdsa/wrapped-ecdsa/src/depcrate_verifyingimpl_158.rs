// Generated macro for impl_158 (impl)
macro_rules! Depcrate_verifyingimpl_158 {
() => {
// Module: crate::verifying
// Provides: {"impl_158"}
// Dependencies: {}
impl < C > PartialEq for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { fn eq (& self , other : & Self) -> bool { self . inner . eq (& other . inner) } }
};
}
