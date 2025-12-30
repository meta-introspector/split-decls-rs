// Generated macro for impl_90 (impl)
macro_rules! Depcrate_projectiveimpl_90 {
() => {
// Module: crate::projective
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < C > BatchNormalize < [ProjectivePoint < C >] > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { type Output = Vec < < Self as CurveGroup > :: AffineRepr > ; # [inline] fn batch_normalize (points : & [Self]) -> Vec < < Self as CurveGroup > :: AffineRepr > { let mut zs = vec ! [C :: FieldElement :: ONE ; points . len ()] ; let mut affine_points = vec ! [AffinePoint :: IDENTITY ; points . len ()] ; batch_normalize_generic (points , zs . as_mut_slice () , & mut affine_points) ; affine_points } }
};
}
