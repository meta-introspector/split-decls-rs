// Generated macro for is_identifier_start (function)
macro_rules! Depcrate_pragmais_identifier_start {
() => {
// Module: crate::pragma
// Provides: {"is_identifier_start"}
// Dependencies: {}
fn is_identifier_start (c : char) -> bool { c . is_ascii_uppercase () || c == '_' || c . is_ascii_lowercase () || c > '\x7F' }
};
}
