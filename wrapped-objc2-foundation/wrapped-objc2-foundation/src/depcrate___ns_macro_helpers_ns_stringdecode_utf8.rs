// Generated macro for decode_utf8 (function)
macro_rules! Depcrate___ns_macro_helpers_ns_stringdecode_utf8 {
() => {
// Module: crate::__ns_macro_helpers::ns_string
// Provides: {"decode_utf8"}
// Dependencies: {}
const fn decode_utf8 (s : & [u8] , i : usize) -> (usize , u32) { let b0 = s [i] ; match b0 { 0b0000_0000 ..= 0b0111_1111 => { let decoded = b0 as u32 ; (i + 1 , decoded) } 0b1100_0000 ..= 0b1101_1111 => { let decoded = ((b0 as u32 & 0x1f) << 6) | (s [i + 1] as u32 & 0x3f) ; (i + 2 , decoded) } 0b1110_0000 ..= 0b1110_1111 => { let decoded = ((b0 as u32 & 0x0f) << 12) | ((s [i + 1] as u32 & 0x3f) << 6) | (s [i + 2] as u32 & 0x3f) ; (i + 3 , decoded) } 0b1111_0000 ..= 0b1111_0111 => { let decoded = ((b0 as u32 & 0x07) << 18) | ((s [i + 1] as u32 & 0x3f) << 12) | ((s [i + 2] as u32 & 0x3f) << 6) | (s [i + 3] as u32 & 0x3f) ; (i + 4 , decoded) } 0b1000_0000 ..= 0b1011_1111 | 0b1111_1000 ..= 0b1111_1111 => { panic ! ("Encountered invalid bytes") } } }
};
}
