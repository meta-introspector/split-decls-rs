// Generated macro for is_identifier (function)
macro_rules! Depcrate_pragmais_identifier {
() => {
// Module: crate::pragma
// Provides: {"is_identifier"}
// Dependencies: {}
fn is_identifier (s : & str) -> bool { let chars = s . char_indices () ; for (i , ch) in chars { if i == 0 { if ! is_identifier_start (ch) { return false ; } } else if ! is_identifier_continue (ch) { return false ; } } true }
};
}
