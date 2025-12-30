// Generated macro for varint_encode_u128 (function)
macro_rules! Depcrate_varint_encode_unsignedvarint_encode_u128 {
() => {
// Module: crate::varint::encode_unsigned
// Provides: {"varint_encode_u128"}
// Dependencies: {}
pub fn varint_encode_u128 < W : Writer > (writer : & mut W , endian : Endianness , val : u128 ,) -> Result < () , EncodeError > { if val <= SINGLE_BYTE_MAX as _ { writer . write (& [val as u8]) } else if val <= u16 :: MAX as _ { writer . write (& [U16_BYTE]) ? ; match endian { Endianness :: Big => writer . write (& (val as u16) . to_be_bytes ()) , Endianness :: Little => writer . write (& (val as u16) . to_le_bytes ()) , } } else if val <= u32 :: MAX as _ { writer . write (& [U32_BYTE]) ? ; match endian { Endianness :: Big => writer . write (& (val as u32) . to_be_bytes ()) , Endianness :: Little => writer . write (& (val as u32) . to_le_bytes ()) , } } else if val <= u64 :: MAX as _ { writer . write (& [U64_BYTE]) ? ; match endian { Endianness :: Big => writer . write (& (val as u64) . to_be_bytes ()) , Endianness :: Little => writer . write (& (val as u64) . to_le_bytes ()) , } } else { writer . write (& [U128_BYTE]) ? ; match endian { Endianness :: Big => writer . write (& val . to_be_bytes ()) , Endianness :: Little => writer . write (& val . to_le_bytes ()) , } } }
};
}
