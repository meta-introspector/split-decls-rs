// Generated macro for eq_delim_args (function)
macro_rules! Depcrate_ast_utilseq_delim_args {
() => {
// Module: crate::ast_utils
// Provides: {"eq_delim_args"}
// Dependencies: {}
pub fn eq_delim_args (l : & DelimArgs , r : & DelimArgs) -> bool { l . delim == r . delim && l . tokens . len () == r . tokens . len () && l . tokens . iter () . zip (r . tokens . iter ()) . all (| (a , b) | a . eq_unspanned (b)) }
};
}
