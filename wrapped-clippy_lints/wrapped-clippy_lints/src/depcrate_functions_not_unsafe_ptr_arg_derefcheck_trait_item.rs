// Generated macro for check_trait_item (function)
macro_rules! Depcrate_functions_not_unsafe_ptr_arg_derefcheck_trait_item {
() => {
// Module: crate::functions::not_unsafe_ptr_arg_deref
// Provides: {"check_trait_item"}
// Dependencies: {}
pub (super) fn check_trait_item < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx hir :: TraitItem < '_ >) { if let hir :: TraitItemKind :: Fn (ref sig , hir :: TraitFn :: Provided (eid)) = item . kind { let body = cx . tcx . hir_body (eid) ; check_raw_ptr (cx , sig . header . safety () , sig . decl , body , item . owner_id . def_id) ; } }
};
}
