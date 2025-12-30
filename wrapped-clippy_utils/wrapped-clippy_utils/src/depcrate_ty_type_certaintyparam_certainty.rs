// Generated macro for param_certainty (function)
macro_rules! Depcrate_ty_type_certaintyparam_certainty {
() => {
// Module: crate::ty::type_certainty
// Provides: {"param_certainty"}
// Dependencies: {}
# [doc = " Tries to tell whether `param` resolves to something certain, e.g., a non-wildcard type if"] # [doc = " present. The certainty `DefId` is cleared before returning."] fn param_certainty (cx : & LateContext < '_ > , param : & Param < '_ >) -> Certainty { let owner_did = cx . tcx . hir_enclosing_body_owner (param . hir_id) ; let Some (fn_decl) = cx . tcx . hir_fn_decl_by_hir_id (cx . tcx . local_def_id_to_hir_id (owner_did)) else { return Certainty :: Uncertain ; } ; let inputs = fn_decl . inputs ; let body_params = cx . tcx . hir_body_owned_by (owner_did) . params ; std :: iter :: zip (body_params , inputs) . find (| (p , _) | p . hir_id == param . hir_id) . map_or (Certainty :: Uncertain , | (_ , ty) | type_certainty (cx , ty) . clear_def_id ()) }
};
}
