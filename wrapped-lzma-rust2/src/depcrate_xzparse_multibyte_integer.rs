// Generated macro for parse_multibyte_integer (function)
macro_rules! Depcrate_xzparse_multibyte_integer {
() => {
// Module: crate::xz
// Provides: {"parse_multibyte_integer"}
// Dependencies: {}
# [doc = " Parse XZ multibyte integer (variable length encoding)."] fn parse_multibyte_integer (data : & [u8]) -> crate :: Result < u64 > { let mut result = 0u64 ; let mut shift = 0 ; for & byte in data { if shift >= 63 { return Err (error_invalid_data ("XZ multibyte integer too large")) ; } result |= ((byte & 0x7F) as u64) << shift ; shift += 7 ; if (byte & 0x80) == 0 { return Ok (result) ; } } Err (error_invalid_data ("incomplete XZ multibyte integer")) }
};
}
