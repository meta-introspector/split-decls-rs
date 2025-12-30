// Generated macro for escape_bytes (function)
macro_rules! Depcrateescape_bytes {
() => {
// Module: crate
// Provides: {"escape_bytes"}
// Dependencies: {}
fn escape_bytes (bytes : & [u8]) -> String { let mut s = String :: new () ; for & b in bytes { s . push_str (& escape_byte (b)) ; } s }
};
}
