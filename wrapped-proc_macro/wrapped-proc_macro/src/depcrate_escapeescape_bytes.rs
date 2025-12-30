// Generated macro for escape_bytes (function)
macro_rules! Depcrate_escapeescape_bytes {
() => {
// Module: crate::escape
// Provides: {"escape_bytes"}
// Dependencies: {}
pub (crate) fn escape_bytes (bytes : & [u8] , opt : EscapeOptions) -> String { let mut repr = String :: new () ; if opt . escape_nonascii { for & byte in bytes { escape_single_byte (byte , opt , & mut repr) ; } } else { let mut chunks = bytes . utf8_chunks () ; while let Some (chunk) = chunks . next () { for ch in chunk . valid () . chars () { escape_single_char (ch , opt , & mut repr) ; } for & byte in chunk . invalid () { escape_single_byte (byte , opt , & mut repr) ; } } } repr }
};
}
