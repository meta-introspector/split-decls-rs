// Generated macro for impl_89 (impl)
macro_rules! Depcrate_projectiveimpl_89 {
() => {
// Module: crate::projective
// Provides: {"impl_89"}
// Dependencies: {}
impl < const N : usize , C > BatchNormalize < [ProjectivePoint < C > ; N] > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { type Output = [< Self as CurveGroup > :: AffineRepr ; N] ; # [inline] fn batch_normalize (points : & [Self ; N]) -> [< Self as CurveGroup > :: AffineRepr ; N] { let zs = [C :: FieldElement :: ONE ; N] ; let mut affine_points = [C :: AffinePoint :: IDENTITY ; N] ; batch_normalize_generic (points , zs , & mut affine_points) ; affine_points } }
};
}
