// Generated macro for is_crate_name (function)
macro_rules! Depcrate_identis_crate_name {
() => {
// Module: crate::ident
// Provides: {"is_crate_name"}
// Dependencies: {}
pub (crate) fn is_crate_name (s : & str) -> bool { let mut cs = s . chars () ; cs . next () . is_some_and (| ch | is_xid_start (ch) || matches ! (ch , '-' | '_')) && cs . all (| ch | is_xid_continue (ch) || matches ! (ch , '-')) }
};
}
