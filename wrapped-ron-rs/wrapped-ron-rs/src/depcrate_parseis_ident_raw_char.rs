// Generated macro for is_ident_raw_char (function)
macro_rules! Depcrate_parseis_ident_raw_char {
() => {
// Module: crate::parse
// Provides: {"is_ident_raw_char"}
// Dependencies: {}
pub fn is_ident_raw_char (c : char) -> bool { matches ! (c , '.' | '+' | '-') | is_xid_continue (c) }
};
}
