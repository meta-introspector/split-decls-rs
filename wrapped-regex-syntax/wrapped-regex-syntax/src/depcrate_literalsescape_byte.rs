// Generated macro for escape_byte (function)
macro_rules! Depcrate_literalsescape_byte {
() => {
// Module: crate::literals
// Provides: {"escape_byte"}
// Dependencies: {}
fn escape_byte (byte : u8) -> String { use std :: ascii :: escape_default ; let escaped : Vec < u8 > = escape_default (byte) . collect () ; String :: from_utf8_lossy (& escaped) . into_owned () }
};
}
