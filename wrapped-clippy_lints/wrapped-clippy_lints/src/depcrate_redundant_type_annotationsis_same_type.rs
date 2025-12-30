// Generated macro for is_same_type (function)
macro_rules! Depcrate_redundant_type_annotationsis_same_type {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"is_same_type"}
// Dependencies: {}
fn is_same_type < 'tcx > (cx : & LateContext < 'tcx > , ty_resolved_path : hir :: def :: Res , func_return_type : Ty < 'tcx >) -> bool { if let hir :: def :: Res :: PrimTy (primty) = ty_resolved_path && func_return_type . is_primitive () && let Some (func_return_type_sym) = func_return_type . primitive_symbol () { return primty . name () == func_return_type_sym ; } if let hir :: def :: Res :: Def (DefKind :: Struct | DefKind :: Union | DefKind :: Enum , defid) = ty_resolved_path && let Some (annotation_ty) = cx . tcx . type_of (defid) . no_bound_vars () { return annotation_ty == func_return_type ; } false }
};
}
