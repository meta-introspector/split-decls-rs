// Generated macro for check (function)
macro_rules! Depcrate_loops_while_floatcheck {
() => {
// Module: crate::loops::while_float
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & rustc_lint :: LateContext < '_ > , condition : & rustc_hir :: Expr < '_ >) { if let ExprKind :: Binary (_op , left , right) = condition . kind && is_float_type (cx , left) && is_float_type (cx , right) { span_lint (cx , super :: WHILE_FLOAT , condition . span , "while condition comparing floats" ,) ; } }
};
}
