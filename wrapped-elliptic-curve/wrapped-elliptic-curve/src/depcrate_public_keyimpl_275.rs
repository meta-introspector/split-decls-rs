// Generated macro for impl_275 (impl)
macro_rules! Depcrate_public_keyimpl_275 {
() => {
// Module: crate::public_key
// Provides: {"impl_275"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > FromEncodedPoint < C > for PublicKey < C > where C : CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { # [doc = " Initialize [`PublicKey`] from an [`EncodedPoint`]"] fn from_encoded_point (encoded_point : & EncodedPoint < C >) -> CtOption < Self > { AffinePoint :: < C > :: from_encoded_point (encoded_point) . and_then (| point | { let is_identity = Choice :: from (u8 :: from (encoded_point . is_identity ())) ; CtOption :: new (PublicKey { point } , ! is_identity) }) } }
};
}
