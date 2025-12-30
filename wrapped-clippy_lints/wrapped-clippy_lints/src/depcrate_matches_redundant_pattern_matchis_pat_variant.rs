// Generated macro for is_pat_variant (function)
macro_rules! Depcrate_matches_redundant_pattern_matchis_pat_variant {
() => {
// Module: crate::matches::redundant_pattern_match
// Provides: {"is_pat_variant"}
// Dependencies: {}
fn is_pat_variant (cx : & LateContext < '_ > , pat : & Pat < '_ > , path : & QPath < '_ > , expected_item : Item) -> bool { let Some (id) = cx . typeck_results () . qpath_res (path , pat . hir_id) . opt_def_id () else { return false ; } ; match expected_item { Item :: Lang (expected_lang_item) => cx . tcx . lang_items () . get (expected_lang_item) . is_some_and (| expected_id | cx . tcx . parent (id) == expected_id) , Item :: Diag (expected_ty , expected_variant) => { let ty = cx . typeck_results () . pat_ty (pat) ; if ty . is_diag_item (cx , expected_ty) { let variant = ty . ty_adt_def () . expect ("struct pattern type is not an ADT") . variant_of_res (cx . qpath_res (path , pat . hir_id)) ; return variant . name == expected_variant ; } false } , } }
};
}
