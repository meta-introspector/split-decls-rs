// Generated macro for varint_encode_i64 (function)
macro_rules! Depcrate_varint_encode_signedvarint_encode_i64 {
() => {
// Module: crate::varint::encode_signed
// Provides: {"varint_encode_i64"}
// Dependencies: {}
pub fn varint_encode_i64 < W : Writer > (writer : & mut W , endian : Endianness , val : i64 ,) -> Result < () , EncodeError > { varint_encode_u64 (writer , endian , if val < 0 { ! (val as u64) * 2 + 1 } else { (val as u64) * 2 } ,) }
};
}
