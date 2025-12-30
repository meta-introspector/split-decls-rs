// Generated macro for char_to_hexdigit (function)
macro_rules! Depcrate_escape_byteschar_to_hexdigit {
() => {
// Module: crate::escape_bytes
// Provides: {"char_to_hexdigit"}
// Dependencies: {}
# [doc = " Convert the given codepoint to its corresponding hexadecimal digit."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics if `ch` is not in `[0-9A-Fa-f]`."] # [cfg (feature = "alloc")] fn char_to_hexdigit (ch : char) -> u8 { u8 :: try_from (ch . to_digit (16) . unwrap ()) . unwrap () }
};
}
