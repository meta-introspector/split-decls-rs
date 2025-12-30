// Generated macro for peel_hir_ty_refs_and_ptrs (function)
macro_rules! Depcratepeel_hir_ty_refs_and_ptrs {
() => {
// Module: crate
// Provides: {"peel_hir_ty_refs_and_ptrs"}
// Dependencies: {}
# [doc = " Returns the base type for HIR references and pointers."] pub fn peel_hir_ty_refs_and_ptrs < 'tcx > (ty : & 'tcx hir :: Ty < 'tcx >) -> & 'tcx hir :: Ty < 'tcx > { match & ty . kind { TyKind :: Ptr (mut_ty) | TyKind :: Ref (_ , mut_ty) => peel_hir_ty_refs_and_ptrs (mut_ty . ty) , _ => ty , } }
};
}
