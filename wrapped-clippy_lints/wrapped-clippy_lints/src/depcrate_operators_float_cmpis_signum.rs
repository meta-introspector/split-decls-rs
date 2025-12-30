// Generated macro for is_signum (function)
macro_rules! Depcrate_operators_float_cmpis_signum {
() => {
// Module: crate::operators::float_cmp
// Provides: {"is_signum"}
// Dependencies: {}
fn is_signum (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let ExprKind :: Unary (UnOp :: Neg , child_expr) = expr . kind { return is_signum (cx , child_expr) ; } if let ExprKind :: MethodCall (method_name , self_arg , [] , _) = expr . kind && method_name . ident . name == sym :: signum { return is_float (cx , self_arg) ; } false }
};
}
