// Generated macro for is_uninit_value_valid_for_ty (function)
macro_rules! Depcrate_tyis_uninit_value_valid_for_ty {
() => {
// Module: crate::ty
// Provides: {"is_uninit_value_valid_for_ty"}
// Dependencies: {}
# [doc = " Checks if a given type looks safe to be uninitialized."] pub fn is_uninit_value_valid_for_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { let typing_env = cx . typing_env () . with_post_analysis_normalized (cx . tcx) ; cx . tcx . check_validity_requirement ((ValidityRequirement :: Uninit , typing_env . as_query_input (ty))) . unwrap_or_else (| _ | is_uninit_value_valid_for_ty_fallback (cx , ty)) }
};
}
