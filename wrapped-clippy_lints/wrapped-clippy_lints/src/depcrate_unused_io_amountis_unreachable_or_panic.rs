// Generated macro for is_unreachable_or_panic (function)
macro_rules! Depcrate_unused_io_amountis_unreachable_or_panic {
() => {
// Module: crate::unused_io_amount
// Provides: {"is_unreachable_or_panic"}
// Dependencies: {}
fn is_unreachable_or_panic (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { let expr = peel_blocks (expr) ; let Some (macro_call) = root_macro_call_first_node (cx , expr) else { return false ; } ; if is_panic (cx , macro_call . def_id) { return ! cx . tcx . hir_is_inside_const_context (expr . hir_id) ; } cx . tcx . is_diagnostic_item (sym :: unreachable_macro , macro_call . def_id) }
};
}
