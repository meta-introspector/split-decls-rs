// Generated macro for varint_decode_u32 (function)
macro_rules! Depcrate_varint_decode_unsignedvarint_decode_u32 {
() => {
// Module: crate::varint::decode_unsigned
// Provides: {"varint_decode_u32"}
// Dependencies: {}
pub fn varint_decode_u32 < R : Reader > (read : & mut R , endian : Endianness) -> Result < u32 , DecodeError > { if let Some (bytes) = read . peek_read (5) { let (discriminant , bytes) = bytes . split_at (1) ; let (out , used) = match discriminant [0] { byte @ 0 ..= SINGLE_BYTE_MAX => (byte as u32 , 1) , U16_BYTE => { let val = match endian { Endianness :: Big => u16 :: from_be_bytes (bytes [.. 2] . try_into () . unwrap ()) , Endianness :: Little => u16 :: from_le_bytes (bytes [.. 2] . try_into () . unwrap ()) , } ; (val as u32 , 3) } U32_BYTE => { let val = match endian { Endianness :: Big => u32 :: from_be_bytes (bytes [.. 4] . try_into () . unwrap ()) , Endianness :: Little => u32 :: from_le_bytes (bytes [.. 4] . try_into () . unwrap ()) , } ; (val , 5) } U64_BYTE => return invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: U64) , U128_BYTE => return invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: U128) , _ => return invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: Reserved) , } ; read . consume (used) ; Ok (out) } else { deserialize_varint_cold_u32 (read , endian) } }
};
}
