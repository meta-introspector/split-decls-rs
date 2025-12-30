// Generated macro for impl_276 (impl)
macro_rules! Depcrate_public_keyimpl_276 {
() => {
// Module: crate::public_key
// Provides: {"impl_276"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > ToEncodedPoint < C > for PublicKey < C > where C : CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { # [doc = " Serialize this [`PublicKey`] as a SEC1 [`EncodedPoint`], optionally applying"] # [doc = " point compression"] fn to_encoded_point (& self , compress : bool) -> EncodedPoint < C > { self . point . to_encoded_point (compress) } }
};
}
