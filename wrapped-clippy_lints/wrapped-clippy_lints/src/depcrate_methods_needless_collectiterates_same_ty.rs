// Generated macro for iterates_same_ty (function)
macro_rules! Depcrate_methods_needless_collectiterates_same_ty {
() => {
// Module: crate::methods::needless_collect
// Provides: {"iterates_same_ty"}
// Dependencies: {}
# [doc = " Checks if `<iter_ty as Iterator>::Item` is the same as `<collect_ty as IntoIter>::Item`"] fn iterates_same_ty < 'tcx > (cx : & LateContext < 'tcx > , iter_ty : Ty < 'tcx > , collect_ty : Ty < 'tcx >) -> bool { if let Some (iter_trait) = cx . tcx . get_diagnostic_item (sym :: Iterator) && let Some (into_iter_trait) = cx . tcx . get_diagnostic_item (sym :: IntoIterator) && let Some (iter_item_ty) = make_normalized_projection (cx . tcx , cx . typing_env () , iter_trait , sym :: Item , [iter_ty]) && let Some (into_iter_item_proj) = make_projection (cx . tcx , into_iter_trait , sym :: Item , [collect_ty]) && let Ok (into_iter_item_ty) = cx . tcx . try_normalize_erasing_regions (cx . typing_env () , Ty :: new_projection_from_args (cx . tcx , into_iter_item_proj . def_id , into_iter_item_proj . args) ,) { iter_item_ty == into_iter_item_ty } else { false } }
};
}
