// Generated macro for is_ident_first_char (function)
macro_rules! Depcrate_parseis_ident_first_char {
() => {
// Module: crate::parse
// Provides: {"is_ident_first_char"}
// Dependencies: {}
pub fn is_ident_first_char (c : char) -> bool { c == '_' || is_xid_start (c) }
};
}
