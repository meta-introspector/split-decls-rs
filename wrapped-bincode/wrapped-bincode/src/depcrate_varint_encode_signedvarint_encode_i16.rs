// Generated macro for varint_encode_i16 (function)
macro_rules! Depcrate_varint_encode_signedvarint_encode_i16 {
() => {
// Module: crate::varint::encode_signed
// Provides: {"varint_encode_i16"}
// Dependencies: {}
pub fn varint_encode_i16 < W : Writer > (writer : & mut W , endian : Endianness , val : i16 ,) -> Result < () , EncodeError > { varint_encode_u16 (writer , endian , if val < 0 { ! (val as u16) * 2 + 1 } else { (val as u16) * 2 } ,) }
};
}
