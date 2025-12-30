// Generated macro for is_inexpensive_expr (function)
macro_rules! Depcrate_significant_drop_tighteningis_inexpensive_expr {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"is_inexpensive_expr"}
// Dependencies: {}
fn is_inexpensive_expr (expr : & hir :: Expr < '_ >) -> bool { let actual = peel_hir_expr_unary (expr) . 0 ; let is_path = matches ! (actual . kind , hir :: ExprKind :: Path (_)) ; let is_lit = matches ! (actual . kind , hir :: ExprKind :: Lit (_)) ; is_path || is_lit }
};
}
