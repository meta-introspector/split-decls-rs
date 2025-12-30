// Generated macro for impl_39 (impl)
macro_rules! Depcrate_affineimpl_39 {
() => {
// Module: crate::affine
// Provides: {"impl_39"}
// Dependencies: {}
impl < C > From < NonIdentity < AffinePoint < C > > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (affine : NonIdentity < AffinePoint < C > >) -> Self { affine . to_point () } }
};
}
