// Generated macro for impl_44 (impl)
macro_rules! Depcrate_serdeimpl_44 {
() => {
// Module: crate::serde
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Signature { fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { struct ByteArrayVisitor ; impl < 'de > de :: Visitor < 'de > for ByteArrayVisitor { type Value = [u8 ; Signature :: BYTE_SIZE] ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("bytestring of length 64") } fn visit_seq < A > (self , mut seq : A) -> Result < [u8 ; Signature :: BYTE_SIZE] , A :: Error > where A : de :: SeqAccess < 'de > , { use de :: Error ; let mut arr = [0u8 ; Signature :: BYTE_SIZE] ; for (i , byte) in arr . iter_mut () . enumerate () { * byte = seq . next_element () ? . ok_or_else (| | Error :: invalid_length (i , & self)) ? ; } Ok (arr) } } deserializer . deserialize_tuple (Signature :: BYTE_SIZE , ByteArrayVisitor) . map (Into :: into) } }
};
}
