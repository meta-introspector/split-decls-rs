// Generated macro for check_sig (function)
macro_rules! Depcrate_iter_not_returning_iteratorcheck_sig {
() => {
// Module: crate::iter_not_returning_iterator
// Provides: {"check_sig"}
// Dependencies: {}
fn check_sig (cx : & LateContext < '_ > , name : Symbol , sig : & FnSig < '_ > , fn_id : LocalDefId) { if sig . decl . implicit_self . has_implicit_self () { let ret_ty = cx . tcx . instantiate_bound_regions_with_erased (cx . tcx . fn_sig (fn_id) . instantiate_identity () . output ()) ; let ret_ty = cx . tcx . try_normalize_erasing_regions (cx . typing_env () , ret_ty) . unwrap_or (ret_ty) ; if cx . tcx . get_diagnostic_item (sym :: Iterator) . is_some_and (| iter_id | ! implements_trait (cx , ret_ty , iter_id , & [])) { span_lint (cx , ITER_NOT_RETURNING_ITERATOR , sig . span , format ! ("this method is named `{name}` but its return type does not implement `Iterator`") ,) ; } } }
};
}
