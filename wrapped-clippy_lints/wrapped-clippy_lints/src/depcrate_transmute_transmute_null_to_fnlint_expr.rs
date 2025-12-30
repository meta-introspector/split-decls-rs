// Generated macro for lint_expr (function)
macro_rules! Depcrate_transmute_transmute_null_to_fnlint_expr {
() => {
// Module: crate::transmute::transmute_null_to_fn
// Provides: {"lint_expr"}
// Dependencies: {}
fn lint_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) { span_lint_and_then (cx , TRANSMUTE_NULL_TO_FN , expr . span , "transmuting a known null pointer into a function pointer" , | diag | { diag . span_label (expr . span , "this transmute results in undefined behavior") ; diag . help ("try wrapping your function pointer type in `Option<T>` instead, and using `None` as a null pointer value") ; } ,) ; }
};
}
