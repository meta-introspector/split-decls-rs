// Generated macro for is_valid_impl_method_candidate (function)
macro_rules! Depcrate_consteval_tests_method_resolutionis_valid_impl_method_candidate {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"is_valid_impl_method_candidate"}
// Dependencies: {}
# [tracing :: instrument (skip_all , fields (name))] fn is_valid_impl_method_candidate (table : & mut InferenceTable < '_ > , self_ty : & Ty , receiver_ty : Option < & Ty > , visible_from_module : Option < ModuleId > , name : Option < & Name > , impl_id : ImplId , item : AssocItemId , item_name : & Name ,) -> IsValidCandidate { match item { AssocItemId :: FunctionId (f) => is_valid_impl_fn_candidate (table , impl_id , f , name , receiver_ty , self_ty , visible_from_module , item_name ,) , AssocItemId :: ConstId (c) => { let db = table . db ; check_that ! (receiver_ty . is_none ()) ; check_that ! (name . is_none_or (| n | n == item_name)) ; if let Some (from_module) = visible_from_module && ! db . assoc_visibility (c . into ()) . is_visible_from (db , from_module) { cov_mark :: hit ! (const_candidate_not_visible) ; return IsValidCandidate :: NotVisible ; } let self_ty_matches = table . run_in_snapshot (| table | { let expected_self_ty = TyBuilder :: impl_self_ty (db , impl_id) . fill_with_inference_vars (table) . build () ; table . unify (& expected_self_ty , self_ty) }) ; if ! self_ty_matches { cov_mark :: hit ! (const_candidate_self_type_mismatch) ; return IsValidCandidate :: No ; } IsValidCandidate :: Yes } _ => IsValidCandidate :: No , } }
};
}
