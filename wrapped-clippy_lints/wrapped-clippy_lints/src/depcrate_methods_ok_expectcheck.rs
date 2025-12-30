// Generated macro for check (function)
macro_rules! Depcrate_methods_ok_expectcheck {
() => {
// Module: crate::methods::ok_expect
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `ok().expect()` for `Result`s"] pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ > , recv_inner : & hir :: Expr < '_ >) { let result_ty = cx . typeck_results () . expr_ty (recv_inner) ; if let Some (error_type) = get_error_type (cx , result_ty) && has_debug_impl (cx , error_type) && let Some (span) = recv . span . trim_start (recv_inner . span) { span_lint_and_then (cx , OK_EXPECT , expr . span , "called `ok().expect()` on a `Result` value" , | diag | { let span = cx . sess () . source_map () . span_extend_while_whitespace (span) ; diag . span_suggestion_verbose (span , "call `expect()` directly on the `Result`" , String :: new () , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
