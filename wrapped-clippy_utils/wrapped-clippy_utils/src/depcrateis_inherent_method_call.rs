// Generated macro for is_inherent_method_call (function)
macro_rules! Depcrateis_inherent_method_call {
() => {
// Module: crate
// Provides: {"is_inherent_method_call"}
// Dependencies: {}
# [doc = " Checks if the given method call expression calls an inherent method."] pub fn is_inherent_method_call (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let Some (method_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { cx . tcx . trait_of_assoc (method_id) . is_none () } else { false } }
};
}
