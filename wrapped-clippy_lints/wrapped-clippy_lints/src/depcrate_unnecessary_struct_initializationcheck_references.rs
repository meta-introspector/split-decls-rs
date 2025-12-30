// Generated macro for check_references (function)
macro_rules! Depcrate_unnecessary_struct_initializationcheck_references {
() => {
// Module: crate::unnecessary_struct_initialization
// Provides: {"check_references"}
// Dependencies: {}
fn check_references (cx : & LateContext < '_ > , expr_a : & Expr < '_ > , expr_b : & Expr < '_ >) -> bool { if let Some (parent) = get_parent_expr (cx , expr_a) && let parent_ty = cx . typeck_results () . expr_ty_adjusted (parent) && parent_ty . is_any_ptr () { if is_copy (cx , cx . typeck_results () . expr_ty (expr_a)) && expr_b . res_local_id () . is_some () { return false ; } if parent_ty . is_mutable_ptr () && ! is_mutable (cx , expr_b) { return false ; } } true }
};
}
