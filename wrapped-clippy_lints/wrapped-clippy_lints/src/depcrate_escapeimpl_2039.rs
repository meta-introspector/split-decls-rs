// Generated macro for impl_2039 (impl)
macro_rules! Depcrate_escapeimpl_2039 {
() => {
// Module: crate::escape
// Provides: {"impl_2039"}
// Dependencies: {}
impl < 'tcx > Delegate < 'tcx > for EscapeDelegate < '_ , 'tcx > { fn consume (& mut self , cmt : & PlaceWithHirId < 'tcx > , _ : HirId) { if cmt . place . projections . is_empty () && let PlaceBase :: Local (lid) = cmt . place . base { self . set . swap_remove (& lid) ; } } fn use_cloned (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn borrow (& mut self , cmt : & PlaceWithHirId < 'tcx > , _ : HirId , _ : ty :: BorrowKind) { if cmt . place . projections . is_empty () && let PlaceBase :: Local (lid) = cmt . place . base { self . set . swap_remove (& lid) ; } } fn mutate (& mut self , cmt : & PlaceWithHirId < 'tcx > , _ : HirId) { if cmt . place . projections . is_empty () && is_argument (self . cx . tcx , cmt . hir_id) { let parent_id = self . cx . tcx . parent_hir_id (cmt . hir_id) ; if let Node :: Expr (..) = self . cx . tcx . parent_hir_node (parent_id) { return ; } if let Some (trait_self_ty) = self . trait_self_ty && self . cx . tcx . hir_name (cmt . hir_id) == kw :: SelfLower && cmt . place . ty () . contains (trait_self_ty) { return ; } if is_non_trait_box (cmt . place . ty ()) && ! self . is_large_box (cmt . place . ty ()) { self . set . insert (cmt . hir_id) ; } } } fn fake_read (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
