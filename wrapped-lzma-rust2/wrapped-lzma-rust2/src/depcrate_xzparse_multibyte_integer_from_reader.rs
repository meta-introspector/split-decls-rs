// Generated macro for parse_multibyte_integer_from_reader (function)
macro_rules! Depcrate_xzparse_multibyte_integer_from_reader {
() => {
// Module: crate::xz
// Provides: {"parse_multibyte_integer_from_reader"}
// Dependencies: {}
fn parse_multibyte_integer_from_reader < R : Read > (reader : & mut R) -> crate :: Result < u64 > { let mut result = 0u64 ; let mut shift = 0 ; for _ in 0 .. 9 { let byte = reader . read_u8 () ? ; if shift >= 63 { return Err (error_invalid_data ("XZ multibyte integer too large")) ; } result |= ((byte & 0x7F) as u64) << shift ; shift += 7 ; if (byte & 0x80) == 0 { return Ok (result) ; } } Err (error_invalid_data ("XZ multibyte integer too long")) }
};
}
