// Generated macro for write_hex_to_vec (function)
macro_rules! Depcrate_functionswrite_hex_to_vec {
() => {
// Module: crate::functions
// Provides: {"write_hex_to_vec"}
// Dependencies: {}
# [inline] pub (crate) fn write_hex_to_vec (e : u8 , output : & mut Vec < u8 >) { output . reserve (6) ; let length = output . len () ; unsafe { output . set_len (length + 6) ; } let output = & mut output [length ..] ; output [0] = b'&' ; output [1] = b'#' ; output [2] = b'x' ; output [5] = b';' ; let he = e >> 4 ; let le = e & 0xF ; output [3] = if he >= 10 { b'A' - 10 + he } else { b'0' + he } ; output [4] = if le >= 10 { b'A' - 10 + le } else { b'0' + le } ; }
};
}
