// Generated macro for decode_int (function)
macro_rules! Depcrate_h3_qpack_decoderdecode_int {
() => {
// Module: crate::h3::qpack::decoder
// Provides: {"decode_int"}
// Dependencies: {}
fn decode_int (b : & mut octets :: Octets , prefix : usize) -> Result < u64 > { let mask = 2u64 . pow (prefix as u32) - 1 ; let mut val = u64 :: from (b . get_u8 () ?) ; val &= mask ; if val < mask { return Ok (val) ; } let mut shift = 0 ; while b . cap () > 0 { let byte = b . get_u8 () ? ; let inc = u64 :: from (byte & 0x7f) . checked_shl (shift) . ok_or (Error :: BufferTooShort) ? ; val = val . checked_add (inc) . ok_or (Error :: BufferTooShort) ? ; shift += 7 ; if byte & 0x80 == 0 { return Ok (val) ; } } Err (Error :: BufferTooShort) }
};
}
