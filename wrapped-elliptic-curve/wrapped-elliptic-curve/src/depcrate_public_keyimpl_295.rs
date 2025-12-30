// Generated macro for impl_295 (impl)
macro_rules! Depcrate_public_keyimpl_295 {
() => {
// Module: crate::public_key
// Provides: {"impl_295"}
// Dependencies: {}
# [cfg (feature = "pem")] impl < C > FromStr for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { type Err = Error ; fn from_str (s : & str) -> Result < Self > { Self :: from_public_key_pem (s) . map_err (| _ | Error) } }
};
}
