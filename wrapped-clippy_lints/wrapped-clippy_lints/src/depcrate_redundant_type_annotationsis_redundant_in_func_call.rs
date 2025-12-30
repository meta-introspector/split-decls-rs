// Generated macro for is_redundant_in_func_call (function)
macro_rules! Depcrate_redundant_type_annotationsis_redundant_in_func_call {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"is_redundant_in_func_call"}
// Dependencies: {}
fn is_redundant_in_func_call < 'tcx > (cx : & LateContext < 'tcx > , ty_resolved_path : hir :: def :: Res , call : & hir :: Expr < 'tcx > ,) -> bool { if let hir :: ExprKind :: Path (init_path) = & call . kind { let func_type = extract_fn_ty (cx , call , init_path) ; if let Some (func_type) = func_type && let Some (init_return_type) = func_ty_to_return_type (cx , func_type) { return is_same_type (cx , ty_resolved_path , init_return_type) ; } } false }
};
}
