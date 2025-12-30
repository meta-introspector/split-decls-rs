// Generated macro for is_function_block (function)
macro_rules! Depcrate_transmute_missing_transmute_annotationsis_function_block {
() => {
// Module: crate::transmute::missing_transmute_annotations
// Provides: {"is_function_block"}
// Dependencies: {}
fn is_function_block (cx : & LateContext < '_ > , expr_hir_id : HirId) -> bool { let def_id = cx . tcx . hir_enclosing_body_owner (expr_hir_id) ; if let Some (body) = cx . tcx . hir_maybe_body_owned_by (def_id) { return body . value . peel_blocks () . hir_id == expr_hir_id ; } false }
};
}
