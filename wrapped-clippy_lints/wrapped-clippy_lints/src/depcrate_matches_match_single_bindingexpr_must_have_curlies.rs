// Generated macro for expr_must_have_curlies (function)
macro_rules! Depcrate_matches_match_single_bindingexpr_must_have_curlies {
() => {
// Module: crate::matches::match_single_binding
// Provides: {"expr_must_have_curlies"}
// Dependencies: {}
fn expr_must_have_curlies (cx : & LateContext < '_ > , match_expr : & Expr < '_ >) -> bool { let parent = cx . tcx . parent_hir_node (match_expr . hir_id) ; if let Node :: Expr (Expr { kind : ExprKind :: Closure (..) | ExprKind :: Binary (..) , .. }) | Node :: AnonConst (..) = parent { return true ; } if let Node :: Arm (arm) = & cx . tcx . parent_hir_node (match_expr . hir_id) && let ExprKind :: Match (..) = arm . body . kind { return true ; } false }
};
}
