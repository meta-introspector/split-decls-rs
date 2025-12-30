// Generated macro for impl_44 (impl)
macro_rules! Depcrate_affineimpl_44 {
() => {
// Module: crate::affine
// Provides: {"impl_44"}
// Dependencies: {}
impl < C > From < AffinePoint < C > > for EncodedPoint < C > where C : PrimeCurveParams , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn from (affine : AffinePoint < C >) -> EncodedPoint < C > { affine . to_encoded_point (false) } }
};
}
