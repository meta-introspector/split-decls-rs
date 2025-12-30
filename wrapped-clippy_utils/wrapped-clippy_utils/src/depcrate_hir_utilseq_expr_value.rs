// Generated macro for eq_expr_value (function)
macro_rules! Depcrate_hir_utilseq_expr_value {
() => {
// Module: crate::hir_utils
// Provides: {"eq_expr_value"}
// Dependencies: {}
# [doc = " Checks if two expressions evaluate to the same value, and don't contain any side effects."] pub fn eq_expr_value (cx : & LateContext < '_ > , left : & Expr < '_ > , right : & Expr < '_ >) -> bool { SpanlessEq :: new (cx) . deny_side_effects () . eq_expr (left , right) }
};
}
