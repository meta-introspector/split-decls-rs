// Generated macro for is_valid_quoted_value (function)
macro_rules! Depcrate_digestis_valid_quoted_value {
() => {
// Module: crate::digest
// Provides: {"is_valid_quoted_value"}
// Dependencies: {}
fn is_valid_quoted_value (s : & str) -> bool { for & b in s . as_bytes () { if char_classes (b) & (C_QDTEXT | C_ESCAPABLE) == 0 { return false ; } } true }
};
}
