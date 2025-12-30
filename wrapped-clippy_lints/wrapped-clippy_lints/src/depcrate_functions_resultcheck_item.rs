// Generated macro for check_item (function)
macro_rules! Depcrate_functions_resultcheck_item {
() => {
// Module: crate::functions::result
// Provides: {"check_item"}
// Dependencies: {}
pub (super) fn check_item < 'tcx > (cx : & LateContext < 'tcx > , item : & hir :: Item < 'tcx > , large_err_threshold : u64 , large_err_ignored : & DefIdSet , msrv : Msrv ,) { if let hir :: ItemKind :: Fn { ref sig , .. } = item . kind && let Some ((hir_ty , err_ty)) = result_err_ty (cx , sig . decl , item . owner_id . def_id , item . span) { if cx . effective_visibilities . is_exported (item . owner_id . def_id) { let fn_header_span = item . span . with_hi (sig . decl . output . span () . hi ()) ; check_result_unit_err (cx , err_ty , fn_header_span , msrv) ; } check_result_large_err (cx , err_ty , hir_ty . span , large_err_threshold , large_err_ignored) ; } }
};
}
