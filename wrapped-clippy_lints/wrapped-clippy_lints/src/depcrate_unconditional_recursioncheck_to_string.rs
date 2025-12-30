// Generated macro for check_to_string (function)
macro_rules! Depcrate_unconditional_recursioncheck_to_string {
() => {
// Module: crate::unconditional_recursion
// Provides: {"check_to_string"}
// Dependencies: {}
fn check_to_string (cx : & LateContext < '_ > , method_span : Span , method_def_id : LocalDefId , name : Ident , expr : & Expr < '_ >) { let args = cx . tcx . instantiate_bound_regions_with_erased (cx . tcx . fn_sig (method_def_id) . skip_binder ()) . inputs () ; if let [_self_arg] = args && let hir_id = cx . tcx . local_def_id_to_hir_id (method_def_id) && let Some ((_ , Node :: Item (Item { kind : ItemKind :: Impl (impl_) , owner_id , .. }) ,)) = cx . tcx . hir_parent_iter (hir_id) . next () && ! cx . tcx . is_automatically_derived (owner_id . to_def_id ()) && let Some (of_trait) = impl_ . of_trait && let Some (trait_def_id) = of_trait . trait_ref . trait_def_id () && cx . tcx . is_diagnostic_item (sym :: ToString , trait_def_id) { let is_bad = match expr . kind { ExprKind :: MethodCall (segment , _receiver , & [_arg] , _) if segment . ident . name == name . name => { if let Some (fn_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && let Some (trait_id) = cx . tcx . trait_of_assoc (fn_id) && trait_id == trait_def_id { true } else { false } } , _ => false , } ; if is_bad { span_error (cx , method_span , expr) ; } } }
};
}
