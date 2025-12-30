// Generated macro for is_contains_sig (function)
macro_rules! Depcrate_methods_needless_collectis_contains_sig {
() => {
// Module: crate::methods::needless_collect
// Provides: {"is_contains_sig"}
// Dependencies: {}
# [doc = " Checks if the given method call matches the expected signature of"] # [doc = " `([&[mut]] self, &<iter_ty as Iterator>::Item) -> bool`"] fn is_contains_sig (cx : & LateContext < '_ > , call_id : HirId , iter_expr : & Expr < '_ >) -> bool { let typeck = cx . typeck_results () ; if let Some (id) = typeck . type_dependent_def_id (call_id) && let sig = cx . tcx . fn_sig (id) . instantiate_identity () && sig . skip_binder () . output () . is_bool () && let [_ , search_ty] = * sig . skip_binder () . inputs () && let ty :: Ref (_ , search_ty , Mutability :: Not) = * cx . tcx . instantiate_bound_regions_with_erased (sig . rebind (search_ty)) . kind () && let Some (iter_trait) = cx . tcx . get_diagnostic_item (sym :: Iterator) && let Some (iter_item) = cx . tcx . associated_items (iter_trait) . find_by_ident_and_kind (cx . tcx , Ident :: with_dummy_span (sym :: Item) , AssocTag :: Type , iter_trait ,) && let args = cx . tcx . mk_args (& [GenericArg :: from (typeck . expr_ty_adjusted (iter_expr))]) && let proj_ty = Ty :: new_projection_from_args (cx . tcx , iter_item . def_id , args) && let Ok (item_ty) = cx . tcx . try_normalize_erasing_regions (cx . typing_env () , proj_ty) { item_ty == EarlyBinder :: bind (search_ty) . instantiate (cx . tcx , cx . typeck_results () . node_args (call_id)) } else { false } }
};
}
