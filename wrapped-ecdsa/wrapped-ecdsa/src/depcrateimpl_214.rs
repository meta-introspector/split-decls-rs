// Generated macro for impl_214 (impl)
macro_rules! Depcrateimpl_214 {
() => {
// Module: crate
// Provides: {"impl_214"}
// Dependencies: {}
impl < C : EcdsaCurve > Zeroize for Signature < C > { fn zeroize (& mut self) { self . r = ScalarValue :: ONE ; self . s = ScalarValue :: ONE ; } }
};
}
