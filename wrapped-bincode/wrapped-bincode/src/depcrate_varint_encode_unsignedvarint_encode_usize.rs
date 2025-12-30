// Generated macro for varint_encode_usize (function)
macro_rules! Depcrate_varint_encode_unsignedvarint_encode_usize {
() => {
// Module: crate::varint::encode_unsigned
// Provides: {"varint_encode_usize"}
// Dependencies: {}
pub fn varint_encode_usize < W : Writer > (writer : & mut W , endian : Endianness , val : usize ,) -> Result < () , EncodeError > { varint_encode_u64 (writer , endian , val as u64) }
};
}
