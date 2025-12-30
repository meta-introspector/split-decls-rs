// Generated macro for impl_7685 (impl)
macro_rules! Depcrate_mut_keyimpl_7685 {
() => {
// Module: crate::mut_key
// Provides: {"impl_7685"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MutableKeyType < 'tcx > { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { if let hir :: ItemKind :: Fn { ref sig , .. } = item . kind { self . check_sig (cx , item . owner_id . def_id , sig . decl) ; } } fn check_impl_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: ImplItem < 'tcx >) { if let hir :: ImplItemKind :: Fn (ref sig , ..) = item . kind && trait_ref_of_method (cx , item . owner_id) . is_none () { self . check_sig (cx , item . owner_id . def_id , sig . decl) ; } } fn check_trait_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: TraitItem < 'tcx >) { if let hir :: TraitItemKind :: Fn (ref sig , ..) = item . kind { self . check_sig (cx , item . owner_id . def_id , sig . decl) ; } } fn check_local (& mut self , cx : & LateContext < 'tcx > , local : & hir :: LetStmt < 'tcx >) { if let hir :: PatKind :: Wild = local . pat . kind { return ; } self . check_ty_ (cx , local . span , cx . typeck_results () . pat_ty (local . pat)) ; } }
};
}
