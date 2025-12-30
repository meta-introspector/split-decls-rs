// Generated macro for impl_82 (impl)
macro_rules! Depcrate_projectiveimpl_82 {
() => {
// Module: crate::projective
// Provides: {"impl_82"}
// Dependencies: {}
impl < C > FromEncodedPoint < C > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { fn from_encoded_point (p : & EncodedPoint < C >) -> CtOption < Self > { AffinePoint :: < C > :: from_encoded_point (p) . map (Self :: from) } }
};
}
