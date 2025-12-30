// Generated macro for is_identifier_continue (function)
macro_rules! Depcrate_pragmais_identifier_continue {
() => {
// Module: crate::pragma
// Provides: {"is_identifier_continue"}
// Dependencies: {}
fn is_identifier_continue (c : char) -> bool { c == '$' || c . is_ascii_digit () || c . is_ascii_uppercase () || c == '_' || c . is_ascii_lowercase () || c > '\x7F' }
};
}
