// Generated macro for impl_42 (impl)
macro_rules! Depcrate_affineimpl_42 {
() => {
// Module: crate::affine
// Provides: {"impl_42"}
// Dependencies: {}
impl < C > From < PublicKey < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (public_key : PublicKey < C >) -> AffinePoint < C > { * public_key . as_affine () } }
};
}
