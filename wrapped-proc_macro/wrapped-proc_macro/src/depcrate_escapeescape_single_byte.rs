// Generated macro for escape_single_byte (function)
macro_rules! Depcrate_escapeescape_single_byte {
() => {
// Module: crate::escape
// Provides: {"escape_single_byte"}
// Dependencies: {}
fn escape_single_byte (byte : u8 , opt : EscapeOptions , repr : & mut String) { if byte == b'\0' { repr . push_str ("\\0") ; } else if (byte == b'\'' && ! opt . escape_single_quote) || (byte == b'"' && ! opt . escape_double_quote) { repr . push (byte as char) ; } else { repr . extend (byte . escape_ascii () . map (char :: from)) ; } }
};
}
