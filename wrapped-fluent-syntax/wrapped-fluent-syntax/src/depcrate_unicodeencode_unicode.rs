// Generated macro for encode_unicode (function)
macro_rules! Depcrate_unicodeencode_unicode {
() => {
// Module: crate::unicode
// Provides: {"encode_unicode"}
// Dependencies: {}
fn encode_unicode (s : Option < & str >) -> char { s . and_then (| s | u32 :: from_str_radix (s , 16) . ok () . and_then (char :: from_u32)) . unwrap_or (UNKNOWN_CHAR) }
};
}
