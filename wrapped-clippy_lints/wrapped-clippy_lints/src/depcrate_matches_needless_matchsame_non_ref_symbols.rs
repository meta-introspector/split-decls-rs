// Generated macro for same_non_ref_symbols (function)
macro_rules! Depcrate_matches_needless_matchsame_non_ref_symbols {
() => {
// Module: crate::matches::needless_match
// Provides: {"same_non_ref_symbols"}
// Dependencies: {}
fn same_non_ref_symbols (pats : & [Pat < '_ >] , exprs : & [Expr < '_ >]) -> bool { if pats . len () != exprs . len () { return false ; } for i in 0 .. pats . len () { if ! pat_same_as_expr (& pats [i] , & exprs [i]) { return false ; } } true }
};
}
