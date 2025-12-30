// Generated macro for check (function)
macro_rules! Depcrate_methods_manual_repeat_ncheck {
() => {
// Module: crate::methods::manual_repeat_n
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , repeat_expr : & Expr < '_ > , take_arg : & Expr < '_ > , msrv : Msrv ,) { if ! expr . span . from_expansion () && cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let ExprKind :: Call (_ , [repeat_arg]) = repeat_expr . kind && let Some (def_id) = fn_def_id (cx , repeat_expr) && cx . tcx . is_diagnostic_item (sym :: iter_repeat , def_id) && ! expr_use_ctxt (cx , expr) . is_ty_unified && let Some (std_or_core) = std_or_core (cx) && msrv . meets (cx , msrvs :: REPEAT_N) { let mut app = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , MANUAL_REPEAT_N , expr . span , "this `repeat().take()` can be written more concisely" , "consider using `repeat_n()` instead" , format ! ("{std_or_core}::iter::repeat_n({}, {})" , snippet_with_context (cx , repeat_arg . span , expr . span . ctxt () , ".." , & mut app) . 0 , snippet (cx , take_arg . span , "..")) , app ,) ; } }
};
}
