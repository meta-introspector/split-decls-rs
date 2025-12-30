// Generated macro for is_ascii_ident (function)
macro_rules! Depcrate_identis_ascii_ident {
() => {
// Module: crate::ident
// Provides: {"is_ascii_ident"}
// Dependencies: {}
pub (crate) fn is_ascii_ident (s : & str) -> bool { let mut cs = s . chars () ; cs . next () . is_some_and (| ch | ch . is_ascii_alphabetic () || matches ! (ch , '_')) && cs . all (| ch | ch . is_ascii_alphanumeric () || matches ! (ch , '_')) }
};
}
