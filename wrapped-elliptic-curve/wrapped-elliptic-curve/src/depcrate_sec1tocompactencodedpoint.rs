// Generated macro for ToCompactEncodedPoint (trait)
macro_rules! Depcrate_sec1ToCompactEncodedPoint {
() => {
// Module: crate::sec1
// Provides: {"ToCompactEncodedPoint"}
// Dependencies: {}
# [doc = " Trait for serializing a value to a SEC1 encoded curve point with compaction."] # [doc = ""] # [doc = " This is intended for use with the `AffinePoint` type for a given elliptic curve."] pub trait ToCompactEncodedPoint < C > where C : Curve , FieldBytesSize < C > : ModulusSize , { # [doc = " Serialize this value as a SEC1 [`EncodedPoint`], optionally applying"] # [doc = " point compression."] fn to_compact_encoded_point (& self) -> CtOption < EncodedPoint < C > > ; }
};
}
