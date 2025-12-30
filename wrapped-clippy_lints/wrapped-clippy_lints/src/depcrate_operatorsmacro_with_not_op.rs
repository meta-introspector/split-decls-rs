// Generated macro for macro_with_not_op (function)
macro_rules! Depcrate_operatorsmacro_with_not_op {
() => {
// Module: crate::operators
// Provides: {"macro_with_not_op"}
// Dependencies: {}
fn macro_with_not_op (e : & Expr < '_ >) -> bool { if let ExprKind :: Unary (_ , e) = e . kind { e . span . from_expansion () } else { false } }
};
}
