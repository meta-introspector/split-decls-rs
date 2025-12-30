// Generated macro for func_hir_id_to_func_ty (function)
macro_rules! Depcrate_redundant_type_annotationsfunc_hir_id_to_func_ty {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"func_hir_id_to_func_ty"}
// Dependencies: {}
fn func_hir_id_to_func_ty < 'tcx > (cx : & LateContext < 'tcx > , hir_id : hir :: hir_id :: HirId) -> Option < Ty < 'tcx > > { if let Some ((defkind , func_defid)) = cx . typeck_results () . type_dependent_def (hir_id) && defkind == DefKind :: AssocFn && let Some (init_ty) = cx . tcx . type_of (func_defid) . no_bound_vars () { Some (init_ty) } else { None } }
};
}
