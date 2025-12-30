// Generated macro for varint_decode_i128 (function)
macro_rules! Depcrate_varint_decode_signedvarint_decode_i128 {
() => {
// Module: crate::varint::decode_signed
// Provides: {"varint_decode_i128"}
// Dependencies: {}
pub fn varint_decode_i128 < R : Reader > (read : & mut R , endian : Endianness ,) -> Result < i128 , DecodeError > { let n = super :: varint_decode_u128 (read , endian) . map_err (DecodeError :: change_integer_type_to_signed) ? ; Ok (if n % 2 == 0 { (n / 2) as _ } else { ! (n / 2) as _ }) }
};
}
