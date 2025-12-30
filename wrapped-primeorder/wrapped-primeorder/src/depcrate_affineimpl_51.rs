// Generated macro for impl_51 (impl)
macro_rules! Depcrate_affineimpl_51 {
() => {
// Module: crate::affine
// Provides: {"impl_51"}
// Dependencies: {}
impl < C > TryFrom < EncodedPoint < C > > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { type Error = Error ; fn try_from (point : EncodedPoint < C >) -> Result < AffinePoint < C > > { AffinePoint :: try_from (& point) } }
};
}
