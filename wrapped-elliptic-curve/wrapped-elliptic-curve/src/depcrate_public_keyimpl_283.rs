// Generated macro for impl_283 (impl)
macro_rules! Depcrate_public_keyimpl_283 {
() => {
// Module: crate::public_key
// Provides: {"impl_283"}
// Dependencies: {}
impl < C > From < PublicKey < C > > for NonIdentity < AffinePoint < C > > where C : CurveArithmetic , { fn from (value : PublicKey < C >) -> Self { Self :: from (& value) } }
};
}
