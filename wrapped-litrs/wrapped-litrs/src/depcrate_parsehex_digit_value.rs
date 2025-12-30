// Generated macro for hex_digit_value (function)
macro_rules! Depcrate_parsehex_digit_value {
() => {
// Module: crate::parse
// Provides: {"hex_digit_value"}
// Dependencies: {}
pub (crate) fn hex_digit_value (digit : u8) -> Option < u8 > { match digit { b'0' ..= b'9' => Some (digit - b'0') , b'a' ..= b'f' => Some (digit - b'a' + 10) , b'A' ..= b'F' => Some (digit - b'A' + 10) , _ => None , } }
};
}
