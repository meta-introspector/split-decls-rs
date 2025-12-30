// Generated macro for ty_has_applicable_get_function (function)
macro_rules! Depcrate_indexing_slicingty_has_applicable_get_function {
() => {
// Module: crate::indexing_slicing
// Provides: {"ty_has_applicable_get_function"}
// Dependencies: {}
# [doc = " Checks if the output Ty of the `get` method on this Ty (if any) matches the Ty returned by the"] # [doc = " indexing operation (if any)."] fn ty_has_applicable_get_function < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx > , array_ty : Ty < 'tcx > , index_expr : & Expr < '_ > ,) -> bool { if let ty :: Adt (_ , _) = array_ty . kind () && let Some (get_output_ty) = get_adt_inherent_method (cx , ty , sym :: get) . map (| m | { cx . tcx . fn_sig (m . def_id) . skip_binder () . output () . skip_binder () }) && let ty :: Adt (def , args) = get_output_ty . kind () && cx . tcx . is_diagnostic_item (sym :: Option , def . 0 . did) && let Some (option_generic_param) = args . first () && let generic_ty = option_generic_param . expect_ty () . peel_refs () && (cx . typeck_results () . expr_ty (index_expr) . peel_refs () == generic_ty . peel_refs () || matches ! (generic_ty . peel_refs () . kind () , ty :: Param (_) | ty :: Alias (_ , _))) { true } else { false } }
};
}
