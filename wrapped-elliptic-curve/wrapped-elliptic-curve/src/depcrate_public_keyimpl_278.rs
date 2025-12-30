// Generated macro for impl_278 (impl)
macro_rules! Depcrate_public_keyimpl_278 {
() => {
// Module: crate::public_key
// Provides: {"impl_278"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > From < & PublicKey < C > > for CompressedPoint < C > where C : CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn from (public_key : & PublicKey < C >) -> CompressedPoint < C > { public_key . to_encoded_point (true) . as_bytes () . try_into () . expect ("wrong compressed point size") } }
};
}
