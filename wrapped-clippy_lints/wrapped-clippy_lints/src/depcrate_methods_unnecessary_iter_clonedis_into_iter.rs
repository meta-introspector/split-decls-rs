// Generated macro for is_into_iter (function)
macro_rules! Depcrate_methods_unnecessary_iter_clonedis_into_iter {
() => {
// Module: crate::methods::unnecessary_iter_cloned
// Provides: {"is_into_iter"}
// Dependencies: {}
# [doc = " Returns true if the named method is `IntoIterator::into_iter`."] pub fn is_into_iter (cx : & LateContext < '_ > , callee_def_id : DefId) -> bool { Some (callee_def_id) == cx . tcx . lang_items () . into_iter_fn () }
};
}
