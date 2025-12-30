// Generated macro for is_feature_name (function)
macro_rules! Depcrate_identis_feature_name {
() => {
// Module: crate::ident
// Provides: {"is_feature_name"}
// Dependencies: {}
pub (crate) fn is_feature_name (s : & str) -> bool { s . chars () . all (| ch | is_xid_continue (ch) || matches ! (ch , '-' | '+' | '.')) }
};
}
