// Generated macro for varint_parse_len (function)
macro_rules! Depcratevarint_parse_len {
() => {
// Module: crate
// Provides: {"varint_parse_len"}
// Dependencies: {}
# [doc = " Returns how long the variable-length integer is, given its first byte."] pub const fn varint_parse_len (first : u8) -> usize { match first >> 6 { 0 => 1 , 1 => 2 , 2 => 4 , 3 => 8 , _ => unreachable ! () , } }
};
}
