// Generated macro for is_some_expr (function)
macro_rules! Depcrate_matches_manual_filteris_some_expr {
() => {
// Module: crate::matches::manual_filter
// Provides: {"is_some_expr"}
// Dependencies: {}
# [doc = " Checks whether <expr> resolves to `Some(target)`"] fn is_some_expr (cx : & LateContext < '_ > , target : HirId , ctxt : SyntaxContext , expr : & Expr < '_ >) -> bool { if let Some (inner_expr) = peels_blocks_incl_unsafe_opt (expr) && let Some (arg) = as_some_expr (cx , inner_expr) { return ctxt == expr . span . ctxt () && arg . res_local_id () == Some (target) ; } false }
};
}
