// Generated macro for span_error (function)
macro_rules! Depcrate_unconditional_recursionspan_error {
() => {
// Module: crate::unconditional_recursion
// Provides: {"span_error"}
// Dependencies: {}
fn span_error (cx : & LateContext < '_ > , method_span : Span , expr : & Expr < '_ >) { span_lint_and_then (cx , UNCONDITIONAL_RECURSION , method_span , "function cannot return without recursing" , | diag | { diag . span_note (expr . span , "recursive call site") ; } ,) ; }
};
}
