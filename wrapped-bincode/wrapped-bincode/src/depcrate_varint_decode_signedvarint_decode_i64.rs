// Generated macro for varint_decode_i64 (function)
macro_rules! Depcrate_varint_decode_signedvarint_decode_i64 {
() => {
// Module: crate::varint::decode_signed
// Provides: {"varint_decode_i64"}
// Dependencies: {}
pub fn varint_decode_i64 < R : Reader > (read : & mut R , endian : Endianness) -> Result < i64 , DecodeError > { let n = super :: varint_decode_u64 (read , endian) . map_err (DecodeError :: change_integer_type_to_signed) ? ; Ok (if n % 2 == 0 { (n / 2) as _ } else { ! (n / 2) as _ }) }
};
}
