// Generated macro for impl_296 (impl)
macro_rules! Depcrate_public_keyimpl_296 {
() => {
// Module: crate::public_key
// Provides: {"impl_296"}
// Dependencies: {}
# [cfg (feature = "pem")] # [allow (clippy :: to_string_trait_impl)] impl < C > ToString for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn to_string (& self) -> String { self . to_public_key_pem (Default :: default ()) . expect ("PEM encoding error") } }
};
}
