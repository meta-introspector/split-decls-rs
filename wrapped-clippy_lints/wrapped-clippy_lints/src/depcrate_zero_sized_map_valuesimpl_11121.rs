// Generated macro for impl_11121 (impl)
macro_rules! Depcrate_zero_sized_map_valuesimpl_11121 {
() => {
// Module: crate::zero_sized_map_values
// Provides: {"impl_11121"}
// Dependencies: {}
impl LateLintPass < '_ > for ZeroSizedMapValues { fn check_ty < 'tcx > (& mut self , cx : & LateContext < 'tcx > , hir_ty : & hir :: Ty < 'tcx , AmbigArg >) { if ! hir_ty . span . from_expansion () && ! in_trait_impl (cx , hir_ty . hir_id) && let ty = ty_from_hir_ty (cx , hir_ty . as_unambig_ty ()) && (is_type_diagnostic_item (cx , ty , sym :: HashMap) || is_type_diagnostic_item (cx , ty , sym :: BTreeMap)) && let ty :: Adt (_ , args) = ty . kind () && let ty = args . type_at (1) && ! ty . has_non_region_param () && ! ty . has_escaping_bound_vars () && let Ok (layout) = cx . layout_of (ty) && layout . is_zst () { span_lint_and_help (cx , ZERO_SIZED_MAP_VALUES , hir_ty . span , "map with zero-sized value type" , None , "consider using a set instead" ,) ; } } }
};
}
