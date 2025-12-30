// Generated macro for is_testing_negative (function)
macro_rules! Depcrate_floating_point_arithmeticis_testing_negative {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"is_testing_negative"}
// Dependencies: {}
# [doc = " See [`is_testing_positive`]"] fn is_testing_negative (cx : & LateContext < '_ > , expr : & Expr < '_ > , test : & Expr < '_ >) -> bool { if let ExprKind :: Binary (Spanned { node : op , .. } , left , right) = expr . kind { match op { BinOpKind :: Gt | BinOpKind :: Ge => is_zero (cx , left , expr . span . ctxt ()) && eq_expr_value (cx , right , test) , BinOpKind :: Lt | BinOpKind :: Le => is_zero (cx , right , expr . span . ctxt ()) && eq_expr_value (cx , left , test) , _ => false , } } else { false } }
};
}
