// Generated macro for check_as_ref (function)
macro_rules! Depcrate_manual_option_as_slicecheck_as_ref {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"check_as_ref"}
// Dependencies: {}
fn check_as_ref (cx : & LateContext < '_ > , expr : & Expr < '_ > , span : Span , msrv : Msrv) { if let ExprKind :: MethodCall (seg , callee , [] , _) = expr . kind && seg . ident . name == sym :: as_ref && cx . typeck_results () . expr_ty (callee) . is_diag_item (cx , sym :: Option) && msrv . meets (cx , if clippy_utils :: is_in_const_context (cx) { msrvs :: CONST_OPTION_AS_SLICE } else { msrvs :: OPTION_AS_SLICE } ,) { span_lint_and_then (cx , MANUAL_OPTION_AS_SLICE , span , "manual implementation of `Option::as_slice`" , | diag | { let mut app = Applicability :: MachineApplicable ; let callee = snippet_with_context (cx , callee . span , expr . span . ctxt () , "_" , & mut app) . 0 ; diag . span_suggestion_verbose (span , "use `Option::as_slice` directly" , format ! ("{callee}.as_slice()") , app ,) ; } ,) ; } }
};
}
