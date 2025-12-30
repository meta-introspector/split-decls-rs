// Generated macro for impl_98 (impl)
macro_rules! Depcrate_projectiveimpl_98 {
() => {
// Module: crate::projective
// Provides: {"impl_98"}
// Dependencies: {}
impl < C > ToEncodedPoint < C > for ProjectivePoint < C > where C : PrimeCurveParams , CompressedPoint < C > : Copy , FieldBytesSize < C > : ModulusSize , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn to_encoded_point (& self , compress : bool) -> EncodedPoint < C > { self . to_affine () . to_encoded_point (compress) } }
};
}
