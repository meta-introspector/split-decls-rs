// Generated macro for impl_41 (impl)
macro_rules! Depcrate_affineimpl_41 {
() => {
// Module: crate::affine
// Provides: {"impl_41"}
// Dependencies: {}
impl < C > From < & ProjectivePoint < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (p : & ProjectivePoint < C >) -> AffinePoint < C > { p . to_affine () } }
};
}
