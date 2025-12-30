// Generated macro for is_char (function)
macro_rules! Depcrate_methods_single_char_add_stris_char {
() => {
// Module: crate::methods::single_char_add_str
// Provides: {"is_char"}
// Dependencies: {}
fn is_char (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (expr) . is_char () }
};
}
