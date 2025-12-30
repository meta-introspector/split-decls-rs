// Generated macro for impl_286 (impl)
macro_rules! Depcrate_public_keyimpl_286 {
() => {
// Module: crate::public_key
// Provides: {"impl_286"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > Ord for PublicKey < C > where C : CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn cmp (& self , other : & Self) -> Ordering { self . to_encoded_point (false) . cmp (& other . to_encoded_point (false)) } }
};
}
