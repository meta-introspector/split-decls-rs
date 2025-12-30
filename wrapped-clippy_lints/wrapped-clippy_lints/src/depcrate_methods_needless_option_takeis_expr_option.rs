// Generated macro for is_expr_option (function)
macro_rules! Depcrate_methods_needless_option_takeis_expr_option {
() => {
// Module: crate::methods::needless_option_take
// Provides: {"is_expr_option"}
// Dependencies: {}
fn is_expr_option (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let expr_type = cx . typeck_results () . expr_ty (expr) ; expr_type . is_diag_item (cx , sym :: Option) }
};
}
