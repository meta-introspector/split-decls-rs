// Generated macro for check_explicit_returns_static_str (function)
macro_rules! Depcrate_unnecessary_literal_boundcheck_explicit_returns_static_str {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"check_explicit_returns_static_str"}
// Dependencies: {}
fn check_explicit_returns_static_str (expr : & Expr < '_ >) -> bool { let mut visitor = FindNonLiteralReturn ; visitor . visit_expr (expr) . is_continue () }
};
}
