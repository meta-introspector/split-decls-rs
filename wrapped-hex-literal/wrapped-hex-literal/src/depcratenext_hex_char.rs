// Generated macro for next_hex_char (function)
macro_rules! Depcratenext_hex_char {
() => {
// Module: crate
// Provides: {"next_hex_char"}
// Dependencies: {}
const fn next_hex_char (string : & [u8] , mut pos : usize) -> Option < (u8 , usize) > { while pos < string . len () { let raw_val = string [pos] ; pos += 1 ; let val = match raw_val { b'0' ..= b'9' => raw_val - 48 , b'A' ..= b'F' => raw_val - 55 , b'a' ..= b'f' => raw_val - 87 , b' ' | b':' | b'\r' | b'\n' | b'\t' => continue , 0 ..= 127 => panic ! ("Encountered invalid ASCII character") , _ => panic ! ("Encountered non-ASCII character") , } ; return Some ((val , pos)) ; } None }
};
}
