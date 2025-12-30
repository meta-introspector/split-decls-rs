// Generated macro for impl_273 (impl)
macro_rules! Depcrate_public_keyimpl_273 {
() => {
// Module: crate::public_key
// Provides: {"impl_273"}
// Dependencies: {}
impl < C > AsRef < AffinePoint < C > > for PublicKey < C > where C : CurveArithmetic , { fn as_ref (& self) -> & AffinePoint < C > { self . as_affine () } }
};
}
