// Generated macro for try_get_inner_pat_and_is_result (function)
macro_rules! Depcrate_option_if_let_elsetry_get_inner_pat_and_is_result {
() => {
// Module: crate::option_if_let_else
// Provides: {"try_get_inner_pat_and_is_result"}
// Dependencies: {}
fn try_get_inner_pat_and_is_result < 'tcx > (cx : & LateContext < 'tcx > , pat : & Pat < 'tcx >) -> Option < (& 'tcx Pat < 'tcx > , bool) > { if let PatKind :: TupleStruct (ref qpath , [inner_pat] , ..) = pat . kind && let res = cx . qpath_res (qpath , pat . hir_id) && let Some (did) = res . ctor_parent (cx) . opt_def_id () { let lang_items = cx . tcx . lang_items () ; if Some (did) == lang_items . option_some_variant () { return Some ((inner_pat , false)) ; } else if Some (did) == lang_items . result_ok_variant () { return Some ((inner_pat , true)) ; } } None }
};
}
