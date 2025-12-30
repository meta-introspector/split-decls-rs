// Generated macro for is_instant_now_call (function)
macro_rules! Depcrate_time_subtractionis_instant_now_call {
() => {
// Module: crate::time_subtraction
// Provides: {"is_instant_now_call"}
// Dependencies: {}
fn is_instant_now_call (cx : & LateContext < '_ > , expr_block : & '_ Expr < '_ >) -> bool { if let ExprKind :: Call (fn_expr , []) = expr_block . kind && cx . ty_based_def (fn_expr) . is_diag_item (cx , sym :: instant_now) { true } else { false } }
};
}
