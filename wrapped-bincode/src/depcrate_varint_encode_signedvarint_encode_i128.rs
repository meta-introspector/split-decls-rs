// Generated macro for varint_encode_i128 (function)
macro_rules! Depcrate_varint_encode_signedvarint_encode_i128 {
() => {
// Module: crate::varint::encode_signed
// Provides: {"varint_encode_i128"}
// Dependencies: {}
pub fn varint_encode_i128 < W : Writer > (writer : & mut W , endian : Endianness , val : i128 ,) -> Result < () , EncodeError > { varint_encode_u128 (writer , endian , if val < 0 { ! (val as u128) * 2 + 1 } else { (val as u128) * 2 } ,) }
};
}
