// Generated macro for is_empty_string (function)
macro_rules! Depcrate_len_zerois_empty_string {
() => {
// Module: crate::len_zero
// Provides: {"is_empty_string"}
// Dependencies: {}
fn is_empty_string (expr : & Expr < '_ >) -> bool { if let ExprKind :: Lit (lit) = expr . kind && let LitKind :: Str (lit , _) = lit . node { let lit = lit . as_str () ; return lit . is_empty () ; } false }
};
}
