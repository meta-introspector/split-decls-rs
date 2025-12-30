// Generated macro for impl_164 (impl)
macro_rules! Depcrate_ecdhimpl_164 {
() => {
// Module: crate::ecdh
// Provides: {"impl_164"}
// Dependencies: {}
impl < C > Zeroize for EphemeralSecret < C > where C : CurveArithmetic , { fn zeroize (& mut self) { self . scalar . zeroize () } }
};
}
