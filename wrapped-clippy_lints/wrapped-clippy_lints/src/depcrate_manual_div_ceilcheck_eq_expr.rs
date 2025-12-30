// Generated macro for check_eq_expr (function)
macro_rules! Depcrate_manual_div_ceilcheck_eq_expr {
() => {
// Module: crate::manual_div_ceil
// Provides: {"check_eq_expr"}
// Dependencies: {}
fn check_eq_expr (cx : & LateContext < '_ > , lhs : & Expr < '_ > , rhs : & Expr < '_ >) -> bool { SpanlessEq :: new (cx) . eq_expr (lhs , rhs) }
};
}
