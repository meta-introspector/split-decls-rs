// Generated macro for check (function)
macro_rules! Depcrate_methods_err_expectcheck {
() => {
// Module: crate::methods::err_expect
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , _expr : & rustc_hir :: Expr < '_ > , recv : & rustc_hir :: Expr < '_ > , expect_span : Span , err_span : Span , msrv : Msrv ,) { let result_ty = cx . typeck_results () . expr_ty (recv) ; if let Some (data_type) = get_data_type (cx , result_ty) && has_debug_impl (cx , data_type) && msrv . meets (cx , msrvs :: EXPECT_ERR) { span_lint_and_sugg (cx , ERR_EXPECT , err_span . to (expect_span) , "called `.err().expect()` on a `Result` value" , "try" , "expect_err" . to_string () , Applicability :: MachineApplicable ,) ; } }
};
}
