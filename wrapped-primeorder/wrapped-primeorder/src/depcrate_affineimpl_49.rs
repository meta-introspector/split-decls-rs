// Generated macro for impl_49 (impl)
macro_rules! Depcrate_affineimpl_49 {
() => {
// Module: crate::affine
// Provides: {"impl_49"}
// Dependencies: {}
impl < C > ToEncodedPoint < C > for AffinePoint < C > where C : PrimeCurveParams , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn to_encoded_point (& self , compress : bool) -> EncodedPoint < C > { EncodedPoint :: < C > :: conditional_select (& EncodedPoint :: < C > :: from_affine_coordinates (& self . x . to_repr () , & self . y . to_repr () , compress ,) , & EncodedPoint :: < C > :: identity () , self . is_identity () ,) } }
};
}
