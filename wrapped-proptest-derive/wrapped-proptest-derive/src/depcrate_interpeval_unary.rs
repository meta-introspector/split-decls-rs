// Generated macro for eval_unary (function)
macro_rules! Depcrate_interpeval_unary {
() => {
// Module: crate::interp
// Provides: {"eval_unary"}
// Dependencies: {}
# [doc = " Interprets unary operator on an expression."] fn eval_unary (expr : & syn :: ExprUnary) -> Option < u128 > { if let U :: Not (_) = expr . op { Some (! eval_expr (& expr . expr) ?) } else { None } }
};
}
