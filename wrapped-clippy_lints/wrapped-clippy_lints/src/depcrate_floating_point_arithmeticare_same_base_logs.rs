// Generated macro for are_same_base_logs (function)
macro_rules! Depcrate_floating_point_arithmeticare_same_base_logs {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"are_same_base_logs"}
// Dependencies: {}
fn are_same_base_logs (cx : & LateContext < '_ > , expr_a : & Expr < '_ > , expr_b : & Expr < '_ >) -> bool { if let ExprKind :: MethodCall (PathSegment { ident : method_a , .. } , _ , args_a , _) = expr_a . kind && let ExprKind :: MethodCall (PathSegment { ident : method_b , .. } , _ , args_b , _) = expr_b . kind { return method_a . name == method_b . name && args_a . len () == args_b . len () && (matches ! (method_a . name , sym :: ln | sym :: log2 | sym :: log10) || method_a . name == sym :: log && args_a . len () == 1 && eq_expr_value (cx , & args_a [0] , & args_b [0])) ; } false }
};
}
