// Generated macro for write_percent_encoded (function)
macro_rules! Depcrate_convertwrite_percent_encoded {
() => {
// Module: crate::convert
// Provides: {"write_percent_encoded"}
// Dependencies: {}
# [doc = " Percent-encodes and writes the IRI string using the given buffer."] fn write_percent_encoded (f : & mut fmt :: Formatter < '_ > , mut s : & str) -> fmt :: Result { while ! s . is_empty () { let non_ascii_pos = s . bytes () . position (| b | ! b . is_ascii ()) . unwrap_or (s . len ()) ; let (ascii , rest) = s . split_at (non_ascii_pos) ; if ! ascii . is_empty () { f . write_str (ascii) ? ; s = rest ; } if s . is_empty () { return Ok (()) ; } let nonascii_end = s . bytes () . position (| b | b . is_ascii ()) . unwrap_or (s . len ()) ; let (nonasciis , rest) = s . split_at (nonascii_end) ; debug_assert ! (! nonasciis . is_empty () , "string without non-ASCII characters should have caused early return") ; s = rest ; # [doc = " Number of source bytes to encode at once."] const NUM_BYTES_AT_ONCE : usize = 21 ; percent_encode_bytes (f , nonasciis , & mut [0_u8 ; NUM_BYTES_AT_ONCE * 3]) ? ; } Ok (()) }
};
}
