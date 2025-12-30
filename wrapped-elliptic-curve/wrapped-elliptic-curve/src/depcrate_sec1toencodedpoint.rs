// Generated macro for ToEncodedPoint (trait)
macro_rules! Depcrate_sec1ToEncodedPoint {
() => {
// Module: crate::sec1
// Provides: {"ToEncodedPoint"}
// Dependencies: {}
# [doc = " Trait for serializing a value to a SEC1 encoded curve point."] # [doc = ""] # [doc = " This is intended for use with the `AffinePoint` type for a given elliptic curve."] pub trait ToEncodedPoint < C > where C : Curve , FieldBytesSize < C > : ModulusSize , { # [doc = " Serialize this value as a SEC1 [`EncodedPoint`], optionally applying"] # [doc = " point compression."] fn to_encoded_point (& self , compress : bool) -> EncodedPoint < C > ; }
};
}
