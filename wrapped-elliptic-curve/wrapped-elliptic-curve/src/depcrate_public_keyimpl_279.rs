// Generated macro for impl_279 (impl)
macro_rules! Depcrate_public_keyimpl_279 {
() => {
// Module: crate::public_key
// Provides: {"impl_279"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > From < PublicKey < C > > for EncodedPoint < C > where C : CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn from (public_key : PublicKey < C >) -> EncodedPoint < C > { EncodedPoint :: < C > :: from (& public_key) } }
};
}
