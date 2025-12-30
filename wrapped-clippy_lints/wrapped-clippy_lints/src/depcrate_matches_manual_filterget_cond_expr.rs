// Generated macro for get_cond_expr (function)
macro_rules! Depcrate_matches_manual_filterget_cond_expr {
() => {
// Module: crate::matches::manual_filter
// Provides: {"get_cond_expr"}
// Dependencies: {}
fn get_cond_expr < 'tcx > (cx : & LateContext < 'tcx > , pat : & Pat < '_ > , expr : & 'tcx Expr < '_ > , ctxt : SyntaxContext ,) -> Option < SomeExpr < 'tcx > > { if let Some (block_expr) = peels_blocks_incl_unsafe_opt (expr) && let ExprKind :: If (cond , then_expr , Some (else_expr)) = block_expr . kind && let PatKind :: Binding (_ , target , ..) = pat . kind && (is_some_expr (cx , target , ctxt , then_expr) && is_none_expr (cx , else_expr) || is_none_expr (cx , then_expr) && is_some_expr (cx , target , ctxt , else_expr)) { return Some (SomeExpr { expr : peels_blocks_incl_unsafe (cond . peel_drop_temps ()) , needs_unsafe_block : contains_unsafe_block (cx , expr) , needs_negated : is_none_expr (cx , then_expr) , }) ; } None }
};
}
