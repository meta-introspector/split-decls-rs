// Generated macro for impl_46 (impl)
macro_rules! Depcrate_serdeimpl_46 {
() => {
// Module: crate::serde
// Provides: {"impl_46"}
// Dependencies: {}
# [cfg (feature = "serde_bytes")] impl < 'de > serde_bytes :: Deserialize < 'de > for Signature { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct ByteArrayVisitor ; impl de :: Visitor < '_ > for ByteArrayVisitor { type Value = SignatureBytes ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("bytestring of length 64") } fn visit_bytes < E > (self , bytes : & [u8]) -> Result < Self :: Value , E > where E : de :: Error , { use de :: Error ; bytes . try_into () . map_err (| _ | Error :: invalid_length (bytes . len () , & self)) } } deserializer . deserialize_bytes (ByteArrayVisitor) . map (Into :: into) } }
};
}
