// Generated macro for opt_ret_type (function)
macro_rules! Depcrate_grammaropt_ret_type {
() => {
// Module: crate::grammar
// Provides: {"opt_ret_type"}
// Dependencies: {}
fn opt_ret_type (p : & mut Parser < '_ >) -> bool { if p . at (T ! [->]) { let m = p . start () ; p . bump (T ! [->]) ; types :: type_no_bounds (p) ; m . complete (p , RET_TYPE) ; true } else { false } }
};
}
