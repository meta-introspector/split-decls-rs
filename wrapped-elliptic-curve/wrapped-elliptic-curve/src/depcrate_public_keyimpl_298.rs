// Generated macro for impl_298 (impl)
macro_rules! Depcrate_public_keyimpl_298 {
() => {
// Module: crate::public_key
// Provides: {"impl_298"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for PublicKey < C > where C : AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let der_bytes = serdect :: slice :: deserialize_hex_or_bin_vec (deserializer) ? ; Self :: from_public_key_der (& der_bytes) . map_err (de :: Error :: custom) } }
};
}
