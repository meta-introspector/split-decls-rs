// Generated macro for is_testing_positive (function)
macro_rules! Depcrate_floating_point_arithmeticis_testing_positive {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"is_testing_positive"}
// Dependencies: {}
# [doc = " Returns true iff expr is an expression which tests whether or not"] # [doc = " test is positive or an expression which tests whether or not test"] # [doc = " is nonnegative."] # [doc = " Used for check-custom-abs function below"] fn is_testing_positive (cx : & LateContext < '_ > , expr : & Expr < '_ > , test : & Expr < '_ >) -> bool { if let ExprKind :: Binary (Spanned { node : op , .. } , left , right) = expr . kind { match op { BinOpKind :: Gt | BinOpKind :: Ge => is_zero (cx , right , expr . span . ctxt ()) && eq_expr_value (cx , left , test) , BinOpKind :: Lt | BinOpKind :: Le => is_zero (cx , left , expr . span . ctxt ()) && eq_expr_value (cx , right , test) , _ => false , } } else { false } }
};
}
