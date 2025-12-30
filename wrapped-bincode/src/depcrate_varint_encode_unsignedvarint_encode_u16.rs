// Generated macro for varint_encode_u16 (function)
macro_rules! Depcrate_varint_encode_unsignedvarint_encode_u16 {
() => {
// Module: crate::varint::encode_unsigned
// Provides: {"varint_encode_u16"}
// Dependencies: {}
pub fn varint_encode_u16 < W : Writer > (writer : & mut W , endian : Endianness , val : u16 ,) -> Result < () , EncodeError > { if val <= SINGLE_BYTE_MAX as _ { writer . write (& [val as u8]) } else { writer . write (& [U16_BYTE]) ? ; match endian { Endianness :: Big => writer . write (& val . to_be_bytes ()) , Endianness :: Little => writer . write (& val . to_le_bytes ()) , } } }
};
}
