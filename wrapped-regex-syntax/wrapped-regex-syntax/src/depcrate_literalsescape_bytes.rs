// Generated macro for escape_bytes (function)
macro_rules! Depcrate_literalsescape_bytes {
() => {
// Module: crate::literals
// Provides: {"escape_bytes"}
// Dependencies: {}
fn escape_bytes (bytes : & [u8]) -> String { let mut s = String :: new () ; for & b in bytes { s . push_str (& escape_byte (b)) ; } s }
};
}
