// Generated macro for check_implicit_returns_static_str (function)
macro_rules! Depcrate_unnecessary_literal_boundcheck_implicit_returns_static_str {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"check_implicit_returns_static_str"}
// Dependencies: {}
fn check_implicit_returns_static_str (body : & Body < '_ >) -> bool { if let ExprKind :: Block (block , _) = body . value . kind && let Some (implicit_ret) = block . expr { return is_str_literal (implicit_ret) ; } false }
};
}
