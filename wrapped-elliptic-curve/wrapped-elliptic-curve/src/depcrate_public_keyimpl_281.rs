// Generated macro for impl_281 (impl)
macro_rules! Depcrate_public_keyimpl_281 {
() => {
// Module: crate::public_key
// Provides: {"impl_281"}
// Dependencies: {}
impl < C , P > From < NonIdentity < P > > for PublicKey < C > where C : CurveArithmetic , P : Copy + Into < AffinePoint < C > > , { fn from (value : NonIdentity < P >) -> Self { Self :: from (& value) } }
};
}
