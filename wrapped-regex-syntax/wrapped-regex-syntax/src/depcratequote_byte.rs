// Generated macro for quote_byte (function)
macro_rules! Depcratequote_byte {
() => {
// Module: crate
// Provides: {"quote_byte"}
// Dependencies: {}
fn quote_byte (b : u8) -> String { if parser :: is_punct (b as char) || b == b'\'' || b == b'"' { quote_char (b as char) } else { let escaped : Vec < u8 > = ascii :: escape_default (b) . collect () ; String :: from_utf8 (escaped) . unwrap () } }
};
}
