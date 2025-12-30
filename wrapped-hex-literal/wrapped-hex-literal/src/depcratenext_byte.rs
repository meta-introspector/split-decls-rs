// Generated macro for next_byte (function)
macro_rules! Depcratenext_byte {
() => {
// Module: crate
// Provides: {"next_byte"}
// Dependencies: {}
const fn next_byte (string : & [u8] , pos : usize) -> Option < (u8 , usize) > { let (half1 , pos) = match next_hex_char (string , pos) { Some (v) => v , None => return None , } ; let (half2 , pos) = match next_hex_char (string , pos) { Some (v) => v , None => panic ! ("Odd number of hex characters") , } ; Some (((half1 << 4) + half2 , pos)) }
};
}
