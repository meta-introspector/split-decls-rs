// Generated macro for check_non_zero_conversion (function)
macro_rules! Depcrate_non_zero_suggestionscheck_non_zero_conversion {
() => {
// Module: crate::non_zero_suggestions
// Provides: {"check_non_zero_conversion"}
// Dependencies: {}
fn check_non_zero_conversion (cx : & LateContext < '_ > , expr : & Expr < '_ > , applicability : Applicability) { if let ExprKind :: Call (func , [arg]) = expr . kind && let ExprKind :: Path (qpath) = & func . kind && let Some (def_id) = cx . qpath_res (qpath , func . hir_id) . opt_def_id () && let ExprKind :: MethodCall (rcv_path , receiver , [] , _) = & arg . kind && rcv_path . ident . name == sym :: get { let fn_name = cx . tcx . item_name (def_id) ; let target_ty = cx . typeck_results () . expr_ty (expr) ; let receiver_ty = cx . typeck_results () . expr_ty (receiver) ; if let ty :: Adt (adt_def , _) = receiver_ty . kind () && adt_def . is_struct () && cx . tcx . get_diagnostic_name (adt_def . did ()) == Some (sym :: NonZero) && let Some (target_non_zero_type) = get_target_non_zero_type (target_ty) { let arg_snippet = get_arg_snippet (cx , arg , rcv_path) ; suggest_non_zero_conversion (cx , expr , fn_name , target_non_zero_type , & arg_snippet , applicability) ; } } }
};
}
