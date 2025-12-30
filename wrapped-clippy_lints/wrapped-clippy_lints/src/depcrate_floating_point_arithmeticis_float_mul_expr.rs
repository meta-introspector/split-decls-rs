// Generated macro for is_float_mul_expr (function)
macro_rules! Depcrate_floating_point_arithmeticis_float_mul_expr {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"is_float_mul_expr"}
// Dependencies: {}
fn is_float_mul_expr < 'a > (cx : & LateContext < '_ > , expr : & 'a Expr < 'a >) -> Option < (& 'a Expr < 'a > , & 'a Expr < 'a >) > { if let ExprKind :: Binary (Spanned { node : BinOpKind :: Mul , .. } , lhs , rhs ,) = & expr . kind && cx . typeck_results () . expr_ty (lhs) . is_floating_point () && cx . typeck_results () . expr_ty (rhs) . is_floating_point () { return Some ((lhs , rhs)) ; } None }
};
}
