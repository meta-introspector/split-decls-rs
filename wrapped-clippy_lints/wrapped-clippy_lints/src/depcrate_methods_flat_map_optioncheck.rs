// Generated macro for check (function)
macro_rules! Depcrate_methods_flat_map_optioncheck {
() => {
// Module: crate::methods::flat_map_option
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , arg : & 'tcx hir :: Expr < '_ > , span : Span) { if ! cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { return ; } let arg_ty = cx . typeck_results () . expr_ty_adjusted (arg) ; let sig = match arg_ty . kind () { ty :: Closure (_ , args) => args . as_closure () . sig () , _ if arg_ty . is_fn () => arg_ty . fn_sig (cx . tcx) , _ => return , } ; if ! sig . output () . skip_binder () . is_diag_item (cx , sym :: Option) { return ; } span_lint_and_sugg (cx , FLAT_MAP_OPTION , span , "used `flat_map` where `filter_map` could be used instead" , "try" , "filter_map" . into () , Applicability :: MachineApplicable ,) ; }
};
}
