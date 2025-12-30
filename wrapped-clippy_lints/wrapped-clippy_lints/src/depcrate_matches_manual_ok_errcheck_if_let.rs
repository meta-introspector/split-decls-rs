// Generated macro for check_if_let (function)
macro_rules! Depcrate_matches_manual_ok_errcheck_if_let {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"check_if_let"}
// Dependencies: {}
pub (crate) fn check_if_let (cx : & LateContext < '_ > , expr : & Expr < '_ > , let_pat : & Pat < '_ > , let_expr : & Expr < '_ > , if_then : & Expr < '_ > , else_expr : & Expr < '_ > ,) { if let Some (inner_expr_ty) = option_arg_ty (cx , cx . typeck_results () . expr_ty (expr)) && let Some ((is_ok , ident)) = is_ok_or_err (cx , let_pat) && is_some_ident (cx , if_then , ident , inner_expr_ty) && is_none (cx , else_expr) { apply_lint (cx , expr , let_expr , is_ok) ; } }
};
}
