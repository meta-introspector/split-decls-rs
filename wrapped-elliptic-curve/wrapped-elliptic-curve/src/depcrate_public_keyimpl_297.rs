// Generated macro for impl_297 (impl)
macro_rules! Depcrate_public_keyimpl_297 {
() => {
// Module: crate::public_key
// Provides: {"impl_297"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < C > Serialize for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { let der = self . to_public_key_der () . map_err (ser :: Error :: custom) ? ; serdect :: slice :: serialize_hex_upper_or_bin (& der , serializer) } }
};
}
