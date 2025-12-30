// Generated macro for impl_40 (impl)
macro_rules! Depcrate_affineimpl_40 {
() => {
// Module: crate::affine
// Provides: {"impl_40"}
// Dependencies: {}
impl < C > From < ProjectivePoint < C > > for AffinePoint < C > where C : PrimeCurveParams , { fn from (p : ProjectivePoint < C >) -> AffinePoint < C > { p . to_affine () } }
};
}
