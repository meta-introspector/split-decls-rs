// Generated macro for parse_lit_int (function)
macro_rules! Depcrate_interpparse_lit_int {
() => {
// Module: crate::interp
// Provides: {"parse_lit_int"}
// Dependencies: {}
# [doc = " Adapted from https://docs.rs/syn/0.14.2/src/syn/lit.rs.html#943 to accept"] # [doc = " u128."] fn parse_lit_int (mut s : & str) -> Option < u128 > { # [doc = " Get the byte at offset idx, or a default of `b'\\0'` if we're looking"] # [doc = " past the end of the input buffer."] pub fn byte < S : AsRef < [u8] > + ? Sized > (s : & S , idx : usize) -> u8 { let s = s . as_ref () ; if idx < s . len () { s [idx] } else { 0 } } let base = match (byte (s , 0) , byte (s , 1)) { (b'0' , b'x') => { s = & s [2 ..] ; 16 } (b'0' , b'o') => { s = & s [2 ..] ; 8 } (b'0' , b'b') => { s = & s [2 ..] ; 2 } (b'0' ..= b'9' , _) => 10 , _ => unreachable ! () , } ; let mut value = 0u128 ; loop { let b = byte (s , 0) ; let digit = match b { b'0' ..= b'9' => u128 :: from (b - b'0') , b'a' ..= b'f' if base > 10 => 10 + u128 :: from (b - b'a') , b'A' ..= b'F' if base > 10 => 10 + u128 :: from (b - b'A') , b'_' => { s = & s [1 ..] ; continue ; } b'.' if base == 10 => return None , b'e' | b'E' if base == 10 => return None , _ => break , } ; if digit >= base { panic ! ("Unexpected digit {:x} out of base range" , digit) ; } value = value . checked_mul (base) ? . checked_add (digit) ? ; s = & s [1 ..] ; } Some (value) }
};
}
