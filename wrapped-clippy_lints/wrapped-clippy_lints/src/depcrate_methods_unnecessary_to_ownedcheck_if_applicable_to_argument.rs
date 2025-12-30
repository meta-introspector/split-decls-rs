// Generated macro for check_if_applicable_to_argument (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedcheck_if_applicable_to_argument {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"check_if_applicable_to_argument"}
// Dependencies: {}
fn check_if_applicable_to_argument < 'tcx > (cx : & LateContext < 'tcx > , arg : & Expr < 'tcx >) { if let ExprKind :: AddrOf (BorrowKind :: Ref , Mutability :: Not , expr) = arg . kind && let ExprKind :: MethodCall (method_path , caller , & [] , _) = expr . kind && let Some (method_def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let method_name = method_path . ident . name && match method_name { sym :: to_owned => cx . tcx . is_diagnostic_item (sym :: to_owned_method , method_def_id) , sym :: to_string => cx . tcx . is_diagnostic_item (sym :: to_string_method , method_def_id) , sym :: to_vec => cx . tcx . impl_of_assoc (method_def_id) . filter (| & impl_did | cx . tcx . type_of (impl_did) . instantiate_identity () . is_slice ()) . is_some () , _ => false , } && let original_arg_ty = cx . typeck_results () . node_type (caller . hir_id) . peel_refs () && let arg_ty = cx . typeck_results () . expr_ty (arg) && let ty :: Ref (_ , arg_ty , Mutability :: Not) = arg_ty . kind () && let arg_ty = arg_ty . peel_refs () && (is_str_and_string (cx , arg_ty , original_arg_ty) || is_slice_and_vec (cx , arg_ty , original_arg_ty)) && let Some (snippet) = caller . span . get_source_text (cx) { span_lint_and_sugg (cx , UNNECESSARY_TO_OWNED , arg . span , format ! ("unnecessary use of `{method_name}`") , "replace it with" , if original_arg_ty . is_array () { format ! ("{snippet}.as_slice()") } else { snippet . to_owned () } , Applicability :: MaybeIncorrect ,) ; } }
};
}
