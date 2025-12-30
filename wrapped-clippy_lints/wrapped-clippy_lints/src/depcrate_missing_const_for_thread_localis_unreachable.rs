// Generated macro for is_unreachable (function)
macro_rules! Depcrate_missing_const_for_thread_localis_unreachable {
() => {
// Module: crate::missing_const_for_thread_local
// Provides: {"is_unreachable"}
// Dependencies: {}
fn is_unreachable (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (macro_call) = macro_backtrace (expr . span) . next () && let Some (diag_name) = cx . tcx . get_diagnostic_name (macro_call . def_id) { return (matches ! (diag_name , sym :: core_panic_macro | sym :: std_panic_macro | sym :: core_panic_2015_macro | sym :: std_panic_2015_macro | sym :: core_panic_2021_macro) && ! cx . tcx . hir_is_inside_const_context (expr . hir_id)) || matches ! (diag_name , sym :: unimplemented_macro | sym :: todo_macro | sym :: unreachable_macro | sym :: unreachable_2015_macro) ; } false }
};
}
