// Generated macro for FromEncodedPoint (trait)
macro_rules! Depcrate_sec1FromEncodedPoint {
() => {
// Module: crate::sec1
// Provides: {"FromEncodedPoint"}
// Dependencies: {}
# [doc = " Trait for deserializing a value from a SEC1 encoded curve point."] # [doc = ""] # [doc = " This is intended for use with the `AffinePoint` type for a given elliptic curve."] pub trait FromEncodedPoint < C > where Self : Sized , C : Curve , FieldBytesSize < C > : ModulusSize , { # [doc = " Deserialize the type this trait is impl'd on from an [`EncodedPoint`]."] fn from_encoded_point (point : & EncodedPoint < C >) -> CtOption < Self > ; }
};
}
