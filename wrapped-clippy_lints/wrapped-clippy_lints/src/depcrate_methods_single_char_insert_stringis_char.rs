// Generated macro for is_char (function)
macro_rules! Depcrate_methods_single_char_insert_stringis_char {
() => {
// Module: crate::methods::single_char_insert_string
// Provides: {"is_char"}
// Dependencies: {}
fn is_char (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { cx . typeck_results () . expr_ty (expr) . is_char () }
};
}
