// Generated macro for check_into_iter_call_arg (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedcheck_into_iter_call_arg {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"check_into_iter_call_arg"}
// Dependencies: {}
# [doc = " Checks whether `expr` is an argument in an `into_iter` call and, if so, determines whether its"] # [doc = " call of a `to_owned`-like function is unnecessary."] fn check_into_iter_call_arg (cx : & LateContext < '_ > , expr : & Expr < '_ > , method_name : Symbol , receiver : & Expr < '_ > , msrv : Msrv ,) -> bool { if let Some (parent) = get_parent_expr (cx , expr) && let Some (callee_def_id) = fn_def_id (cx , parent) && is_into_iter (cx , callee_def_id) && let Some (iterator_trait_id) = cx . tcx . get_diagnostic_item (sym :: Iterator) && let parent_ty = cx . typeck_results () . expr_ty (parent) && implements_trait (cx , parent_ty , iterator_trait_id , & []) && let Some (item_ty) = get_iterator_item_ty (cx , parent_ty) && let Some (receiver_snippet) = receiver . span . get_source_text (cx) && ! cx . typeck_results () . expr_ty (receiver) . is_diag_item (cx , sym :: Cow) && ! is_expr_temporary_value (cx , receiver) { if unnecessary_iter_cloned :: check_for_loop_iter (cx , parent , method_name , receiver , true) { return true ; } let cloned_or_copied = if is_copy (cx , item_ty) && msrv . meets (cx , msrvs :: ITERATOR_COPIED) { "copied" } else { "cloned" } ; span_lint_and_sugg (cx , UNNECESSARY_TO_OWNED , parent . span , format ! ("unnecessary use of `{method_name}`") , "use" , format ! ("{receiver_snippet}.iter().{cloned_or_copied}()") , Applicability :: MaybeIncorrect ,) ; return true ; } false }
};
}
