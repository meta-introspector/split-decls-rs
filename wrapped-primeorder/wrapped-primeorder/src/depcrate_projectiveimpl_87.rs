// Generated macro for impl_87 (impl)
macro_rules! Depcrate_projectiveimpl_87 {
() => {
// Module: crate::projective
// Provides: {"impl_87"}
// Dependencies: {}
impl < C > PrimeCurve for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , CompressedPoint < C > : Copy + Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { type Affine = AffinePoint < C > ; }
};
}
