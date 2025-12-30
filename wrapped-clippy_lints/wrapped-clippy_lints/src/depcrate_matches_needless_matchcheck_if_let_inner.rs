// Generated macro for check_if_let_inner (function)
macro_rules! Depcrate_matches_needless_matchcheck_if_let_inner {
() => {
// Module: crate::matches::needless_match
// Provides: {"check_if_let_inner"}
// Dependencies: {}
fn check_if_let_inner (cx : & LateContext < '_ > , if_let : & higher :: IfLet < '_ >) -> bool { if let Some (if_else) = if_let . if_else { if ! pat_same_as_expr (if_let . let_pat , peel_blocks_with_stmt (if_let . if_then)) { return false ; } if let Some (ref nested_if_let) = higher :: IfLet :: hir (cx , if_else) && SpanlessEq :: new (cx) . eq_expr (nested_if_let . let_expr , if_let . let_expr) { return check_if_let_inner (cx , nested_if_let) ; } if matches ! (if_else . kind , ExprKind :: Block (..)) { let else_expr = peel_blocks_with_stmt (if_else) ; if matches ! (else_expr . kind , ExprKind :: Block (..)) { return false ; } let let_expr_ty = cx . typeck_results () . expr_ty (if_let . let_expr) ; if let_expr_ty . is_diag_item (cx , sym :: Option) { return else_expr . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionNone) || eq_expr_value (cx , if_let . let_expr , else_expr) ; } return eq_expr_value (cx , if_let . let_expr , else_expr) ; } } false }
};
}
