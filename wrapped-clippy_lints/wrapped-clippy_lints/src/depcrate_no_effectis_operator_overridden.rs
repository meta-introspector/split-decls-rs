// Generated macro for is_operator_overridden (function)
macro_rules! Depcrate_no_effectis_operator_overridden {
() => {
// Module: crate::no_effect
// Provides: {"is_operator_overridden"}
// Dependencies: {}
fn is_operator_overridden (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Binary (..) | ExprKind :: Unary (..) => { cx . typeck_results () . is_method_call (expr) } , _ => false , } }
};
}
