// Generated macro for is_expr_temporary_value (function)
macro_rules! Depcrateis_expr_temporary_value {
() => {
// Module: crate
// Provides: {"is_expr_temporary_value"}
// Dependencies: {}
# [doc = " Checks if the expression is a temporary value."] pub fn is_expr_temporary_value (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { ! expr . is_place_expr (| base | { cx . typeck_results () . adjustments () . get (base . hir_id) . is_some_and (| x | x . iter () . any (| adj | matches ! (adj . kind , Adjust :: Deref (_)))) }) }
};
}
