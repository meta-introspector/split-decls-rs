// Generated macro for is_in_panic_handler (function)
macro_rules! Depcrateis_in_panic_handler {
() => {
// Module: crate
// Provides: {"is_in_panic_handler"}
// Dependencies: {}
# [doc = " Returns `true` if the expression is in the program's `#[panic_handler]`."] pub fn is_in_panic_handler (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { let parent = cx . tcx . hir_get_parent_item (e . hir_id) ; Some (parent . to_def_id ()) == cx . tcx . lang_items () . panic_impl () }
};
}
