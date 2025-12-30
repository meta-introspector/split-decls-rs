// Generated macro for get_parent_call_exprs (function)
macro_rules! Depcrate_redundant_closure_callget_parent_call_exprs {
() => {
// Module: crate::redundant_closure_call
// Provides: {"get_parent_call_exprs"}
// Dependencies: {}
# [doc = " \"Walks up\" the chain of calls to find the outermost call expression, and returns the depth:"] # [doc = " ```rust,ignore"] # [doc = " (|| || || 3)()()()"] # [doc = "             ^^      this is the call expression we were given"] # [doc = "                 ^^  this is what we want to return (and the depth is 3)"] # [doc = " ```"] fn get_parent_call_exprs < 'tcx > (cx : & LateContext < 'tcx > , mut expr : & 'tcx hir :: Expr < 'tcx > ,) -> (& 'tcx hir :: Expr < 'tcx > , usize) { let mut depth = 1 ; while let Some (parent) = get_parent_expr (cx , expr) && let ExprKind :: Call (recv , _) = parent . kind && expr . span == recv . span { expr = parent ; depth += 1 ; } (expr , depth) }
};
}
