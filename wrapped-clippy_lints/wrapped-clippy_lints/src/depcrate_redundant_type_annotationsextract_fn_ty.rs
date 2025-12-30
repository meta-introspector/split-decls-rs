// Generated macro for extract_fn_ty (function)
macro_rules! Depcrate_redundant_type_annotationsextract_fn_ty {
() => {
// Module: crate::redundant_type_annotations
// Provides: {"extract_fn_ty"}
// Dependencies: {}
# [doc = " Extracts the fn Ty, e.g. `fn() -> std::string::String {f}`"] fn extract_fn_ty < 'tcx > (cx : & LateContext < 'tcx > , call : & hir :: Expr < 'tcx > , func_return_path : & hir :: QPath < 'tcx > ,) -> Option < Ty < 'tcx > > { match func_return_path { hir :: QPath :: Resolved (_ , resolved_path) => { if let hir :: def :: Res :: Def (_ , defid) = resolved_path . res && let Some (middle_ty_init) = cx . tcx . type_of (defid) . no_bound_vars () { Some (middle_ty_init) } else { None } } , hir :: QPath :: TypeRelative (..) => func_hir_id_to_func_ty (cx , call . hir_id) , } }
};
}
