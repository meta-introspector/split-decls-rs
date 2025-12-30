// Generated macro for is_empty_array (function)
macro_rules! Depcrate_len_zerois_empty_array {
() => {
// Module: crate::len_zero
// Provides: {"is_empty_array"}
// Dependencies: {}
fn is_empty_array (expr : & Expr < '_ >) -> bool { if let ExprKind :: Array (arr) = expr . kind { return arr . is_empty () ; } false }
};
}
