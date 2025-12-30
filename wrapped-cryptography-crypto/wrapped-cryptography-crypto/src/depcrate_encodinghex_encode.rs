// Generated macro for hex_encode (function)
macro_rules! Depcrate_encodinghex_encode {
() => {
// Module: crate::encoding
// Provides: {"hex_encode"}
// Dependencies: {}
pub fn hex_encode (data : & [u8]) -> String { const HEX_CHARS : & [u8 ; 16] = b"0123456789ABCDEF" ; let mut result = String :: with_capacity (data . len () * 2) ; for & byte in data { result . push (HEX_CHARS [(byte >> 4) as usize] as char) ; result . push (HEX_CHARS [(byte & 0x0F) as usize] as char) ; } result }
};
}
