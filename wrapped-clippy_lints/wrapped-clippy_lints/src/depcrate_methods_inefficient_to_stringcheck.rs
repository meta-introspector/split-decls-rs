// Generated macro for check (function)
macro_rules! Depcrate_methods_inefficient_to_stringcheck {
() => {
// Module: crate::methods::inefficient_to_string
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , receiver : & hir :: Expr < '_ > , msrv : Msrv) { if let Some (to_string_meth_did) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && cx . tcx . is_diagnostic_item (sym :: to_string_method , to_string_meth_did) && let Some (args) = cx . typeck_results () . node_args_opt (expr . hir_id) && let arg_ty = cx . typeck_results () . expr_ty_adjusted (receiver) && let self_ty = args . type_at (0) && let (deref_self_ty , deref_count , _) = peel_and_count_ty_refs (self_ty) && deref_count >= 1 && ! msrv . meets (cx , msrvs :: SPECIALIZED_TO_STRING_FOR_REFS) && specializes_tostring (cx , deref_self_ty) { span_lint_and_then (cx , INEFFICIENT_TO_STRING , expr . span , format ! ("calling `to_string` on `{arg_ty}`") , | diag | { diag . help (format ! ("`{self_ty}` implements `ToString` through a slower blanket impl, but `{deref_self_ty}` has a fast specialization of `ToString`")) ; let mut applicability = Applicability :: MachineApplicable ; let arg_snippet = snippet_with_applicability (cx , receiver . span , ".." , & mut applicability) ; diag . span_suggestion (expr . span , "try dereferencing the receiver" , format ! ("({}{arg_snippet}).to_string()" , "*" . repeat (deref_count)) , applicability ,) ; } ,) ; } }
};
}
