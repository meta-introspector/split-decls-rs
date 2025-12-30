// Generated macro for is_instant_now_call (function)
macro_rules! Depcrate_instant_subtractionis_instant_now_call {
() => {
// Module: crate::instant_subtraction
// Provides: {"is_instant_now_call"}
// Dependencies: {}
fn is_instant_now_call (cx : & LateContext < '_ > , expr_block : & '_ Expr < '_ >) -> bool { if let ExprKind :: Call (fn_expr , []) = expr_block . kind && is_path_diagnostic_item (cx , fn_expr , sym :: instant_now) { true } else { false } }
};
}
