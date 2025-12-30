// Generated macro for is_is_empty_sig (function)
macro_rules! Depcrate_methods_needless_collectis_is_empty_sig {
() => {
// Module: crate::methods::needless_collect
// Provides: {"is_is_empty_sig"}
// Dependencies: {}
# [doc = " Checks if the given method call matches the expected signature of `([&[mut]] self) -> bool`"] fn is_is_empty_sig (cx : & LateContext < '_ > , call_id : HirId) -> bool { cx . typeck_results () . type_dependent_def_id (call_id) . is_some_and (| id | { let sig = cx . tcx . fn_sig (id) . instantiate_identity () . skip_binder () ; sig . inputs () . len () == 1 && sig . output () . is_bool () }) }
};
}
