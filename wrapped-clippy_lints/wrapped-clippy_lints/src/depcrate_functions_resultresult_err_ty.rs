// Generated macro for result_err_ty (function)
macro_rules! Depcrate_functions_resultresult_err_ty {
() => {
// Module: crate::functions::result
// Provides: {"result_err_ty"}
// Dependencies: {}
# [doc = " The type of the `Err`-variant in a `std::result::Result` returned by the"] # [doc = " given `FnDecl`"] fn result_err_ty < 'tcx > (cx : & LateContext < 'tcx > , decl : & hir :: FnDecl < 'tcx > , id : hir :: def_id :: LocalDefId , item_span : Span ,) -> Option < (& 'tcx hir :: Ty < 'tcx > , Ty < 'tcx >) > { if ! item_span . in_external_macro (cx . sess () . source_map ()) && let hir :: FnRetTy :: Return (hir_ty) = decl . output && let ty = cx . tcx . instantiate_bound_regions_with_erased (cx . tcx . fn_sig (id) . instantiate_identity () . output ()) && ty . is_diag_item (cx , sym :: Result) && let ty :: Adt (_ , args) = ty . kind () { let err_ty = args . type_at (1) ; Some ((hir_ty , err_ty)) } else { None } }
};
}
