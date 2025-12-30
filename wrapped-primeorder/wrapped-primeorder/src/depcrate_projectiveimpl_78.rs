// Generated macro for impl_78 (impl)
macro_rules! Depcrate_projectiveimpl_78 {
() => {
// Module: crate::projective
// Provides: {"impl_78"}
// Dependencies: {}
impl < C > From < & AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (p : & AffinePoint < C >) -> Self { Self :: from (* p) } }
};
}
