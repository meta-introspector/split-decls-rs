// Generated macro for impl_280 (impl)
macro_rules! Depcrate_public_keyimpl_280 {
() => {
// Module: crate::public_key
// Provides: {"impl_280"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > From < & PublicKey < C > > for EncodedPoint < C > where C : CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn from (public_key : & PublicKey < C >) -> EncodedPoint < C > { public_key . to_encoded_point (C :: COMPRESS_POINTS) } }
};
}
