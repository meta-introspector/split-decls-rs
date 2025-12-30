// Generated macro for escaped_expr_string (function)
macro_rules! Depcrate_constructescaped_expr_string {
() => {
// Module: crate::construct
// Provides: {"escaped_expr_string"}
// Dependencies: {}
pub (crate) fn escaped_expr_string (expr : & Expr) -> String { quote ! (# expr) . to_string () . replace ('{' , "{{") . replace ('}' , "}}") }
};
}
