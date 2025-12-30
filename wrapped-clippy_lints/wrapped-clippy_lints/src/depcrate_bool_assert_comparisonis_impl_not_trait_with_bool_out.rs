// Generated macro for is_impl_not_trait_with_bool_out (function)
macro_rules! Depcrate_bool_assert_comparisonis_impl_not_trait_with_bool_out {
() => {
// Module: crate::bool_assert_comparison
// Provides: {"is_impl_not_trait_with_bool_out"}
// Dependencies: {}
fn is_impl_not_trait_with_bool_out < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { cx . tcx . lang_items () . not_trait () . filter (| trait_id | implements_trait (cx , ty , * trait_id , & [])) . and_then (| trait_id | { cx . tcx . associated_items (trait_id) . find_by_ident_and_kind (cx . tcx , Ident :: with_dummy_span (sym :: Output) , ty :: AssocTag :: Type , trait_id ,) }) . is_some_and (| assoc_item | { let proj = Ty :: new_projection (cx . tcx , assoc_item . def_id , cx . tcx . mk_args_trait (ty , [])) ; let nty = cx . tcx . normalize_erasing_regions (cx . typing_env () , proj) ; nty . is_bool () }) }
};
}
