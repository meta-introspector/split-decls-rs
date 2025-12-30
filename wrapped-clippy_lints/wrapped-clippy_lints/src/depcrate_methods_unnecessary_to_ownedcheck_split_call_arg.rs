// Generated macro for check_split_call_arg (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedcheck_split_call_arg {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"check_split_call_arg"}
// Dependencies: {}
# [doc = " Checks whether `expr` is an argument in an `into_iter` call and, if so, determines whether its"] # [doc = " call of a `to_owned`-like function is unnecessary."] fn check_split_call_arg (cx : & LateContext < '_ > , expr : & Expr < '_ > , method_name : Symbol , receiver : & Expr < '_ >) -> bool { if let Some (parent) = get_parent_expr (cx , expr) && let Some ((sym :: split , argument_expr)) = get_fn_name_and_arg (cx , parent) && let Some (receiver_snippet) = receiver . span . get_source_text (cx) && let Some (arg_snippet) = argument_expr . span . get_source_text (cx) { let as_ref = if cx . typeck_results () . expr_ty (expr) . is_lang_item (cx , LangItem :: String) && let Some (deref_trait_id) = cx . tcx . get_diagnostic_item (sym :: Deref) && cx . get_associated_type (cx . typeck_results () . expr_ty (receiver) , deref_trait_id , sym :: Target) != Some (cx . tcx . types . str_) { ".as_ref()" } else { "" } ; span_lint_and_sugg (cx , UNNECESSARY_TO_OWNED , parent . span , format ! ("unnecessary use of `{method_name}`") , "use" , format ! ("{receiver_snippet}{as_ref}.split({arg_snippet})") , Applicability :: MaybeIncorrect ,) ; return true ; } false }
};
}
