// Generated macro for impl_136 (impl)
macro_rules! Depcrate_outer_expn_data_passimpl_136 {
() => {
// Module: crate::outer_expn_data_pass
// Provides: {"impl_136"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for OuterExpnDataPass { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { if is_lint_allowed (cx , OUTER_EXPN_EXPN_DATA , expr . hir_id) { return ; } let (method_names , arg_lists , spans) = method_calls (expr , 2) ; if let [sym :: expn_data , sym :: outer_expn] = method_names . as_slice () && let (self_arg , args) = arg_lists [1] && args . is_empty () && let self_ty = cx . typeck_results () . expr_ty (self_arg) . peel_refs () && internal_paths :: SYNTAX_CONTEXT . matches_ty (cx , self_ty) { span_lint_and_sugg (cx , OUTER_EXPN_EXPN_DATA , spans [1] . with_hi (expr . span . hi ()) , "usage of `outer_expn().expn_data()`" , "try" , "outer_expn_data()" . to_string () , Applicability :: MachineApplicable ,) ; } } }
};
}
