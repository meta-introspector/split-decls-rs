// Generated macro for varint_encode_i32 (function)
macro_rules! Depcrate_varint_encode_signedvarint_encode_i32 {
() => {
// Module: crate::varint::encode_signed
// Provides: {"varint_encode_i32"}
// Dependencies: {}
pub fn varint_encode_i32 < W : Writer > (writer : & mut W , endian : Endianness , val : i32 ,) -> Result < () , EncodeError > { varint_encode_u32 (writer , endian , if val < 0 { ! (val as u32) * 2 + 1 } else { (val as u32) * 2 } ,) }
};
}
