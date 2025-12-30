// Generated macro for decode_dirty (function)
macro_rules! Depcrate_hexdecode_dirty {
() => {
// Module: crate::hex
// Provides: {"decode_dirty"}
// Dependencies: {}
# [doc = " Converts a hex string to a vector of bytes."] # [doc = " It ignores any characters that are not valid hex digits."] # [must_use] # [allow (clippy :: missing_panics_doc)] pub fn decode_dirty (hex_str : & str) -> Vec < u8 > { let clean : String = hex_str . chars () . filter (char :: is_ascii_hexdigit) . collect () ; decode (clean . as_str ()) . unwrap () }
};
}
