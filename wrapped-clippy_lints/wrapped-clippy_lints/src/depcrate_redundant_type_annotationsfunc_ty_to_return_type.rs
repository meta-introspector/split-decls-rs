// Generated macro for func_ty_to_return_type (function)
macro_rules! Depcrate_redundant_type_annotationsfunc_ty_to_return_type {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"func_ty_to_return_type"}
// Dependencies: {}
fn func_ty_to_return_type < 'tcx > (cx : & LateContext < 'tcx > , func_ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { if func_ty . is_fn () { Some (func_ty . fn_sig (cx . tcx) . output () . skip_binder ()) } else { None } }
};
}
