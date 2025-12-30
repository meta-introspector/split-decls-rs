// Generated macro for check_expr_range (function)
macro_rules! Depcrate_manual_is_ascii_checkcheck_expr_range {
() => {
// Module: crate::manual_is_ascii_check
// Provides: {"check_expr_range"}
// Dependencies: {}
fn check_expr_range (start : & Expr < '_ > , end : & Expr < '_ >) -> CharRange { if let ExprKind :: Lit (start_lit) = & start . kind && let ExprKind :: Lit (end_lit) = & end . kind { check_lit_range (start_lit , end_lit) } else { CharRange :: Otherwise } }
};
}
