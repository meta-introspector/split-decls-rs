// Generated macro for impl_277 (impl)
macro_rules! Depcrate_public_keyimpl_277 {
() => {
// Module: crate::public_key
// Provides: {"impl_277"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > From < PublicKey < C > > for CompressedPoint < C > where C : CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn from (public_key : PublicKey < C >) -> CompressedPoint < C > { CompressedPoint :: < C > :: from (& public_key) } }
};
}
