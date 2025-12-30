// Generated macro for is_ident (function)
macro_rules! Depcrate_identis_ident {
() => {
// Module: crate::ident
// Provides: {"is_ident"}
// Dependencies: {}
pub (crate) fn is_ident (s : & str) -> bool { let mut cs = s . chars () ; cs . next () . is_some_and (| ch | is_xid_start (ch) || matches ! (ch , '_')) && cs . all (is_xid_continue) }
};
}
