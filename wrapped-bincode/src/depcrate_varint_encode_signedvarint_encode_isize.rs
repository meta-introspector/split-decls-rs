// Generated macro for varint_encode_isize (function)
macro_rules! Depcrate_varint_encode_signedvarint_encode_isize {
() => {
// Module: crate::varint::encode_signed
// Provides: {"varint_encode_isize"}
// Dependencies: {}
pub fn varint_encode_isize < W : Writer > (writer : & mut W , endian : Endianness , val : isize ,) -> Result < () , EncodeError > { varint_encode_i64 (writer , endian , val as i64) }
};
}
