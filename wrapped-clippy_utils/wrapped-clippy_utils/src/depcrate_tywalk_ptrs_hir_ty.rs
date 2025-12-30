// Generated macro for walk_ptrs_hir_ty (function)
macro_rules! Depcrate_tywalk_ptrs_hir_ty {
() => {
// Module: crate::ty
// Provides: {"walk_ptrs_hir_ty"}
// Dependencies: {}
# [doc = " Returns the base type for HIR references and pointers."] pub fn walk_ptrs_hir_ty < 'tcx > (ty : & 'tcx hir :: Ty < 'tcx >) -> & 'tcx hir :: Ty < 'tcx > { match ty . kind { TyKind :: Ptr (ref mut_ty) | TyKind :: Ref (_ , ref mut_ty) => walk_ptrs_hir_ty (mut_ty . ty) , _ => ty , } }
};
}
