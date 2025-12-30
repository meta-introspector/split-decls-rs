// Generated macro for impl_11223 (impl)
macro_rules! Depcrate_unwrapimpl_11223 {
() => {
// Module: crate::unwrap
// Provides: {"impl_11223"}
// Dependencies: {}
impl < 'tcx > Delegate < 'tcx > for MutationVisitor < 'tcx , '_ > { fn borrow (& mut self , cat : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId , bk : ty :: BorrowKind) { if let ty :: BorrowKind :: Mutable = bk && self . local . is_potentially_local_place (& cat . place) && ! is_as_mut_use (self . tcx , diag_expr_id) { self . is_mutated = true ; } } fn mutate (& mut self , cat : & PlaceWithHirId < 'tcx > , _ : HirId) { if self . local . is_potentially_local_place (& cat . place) { self . is_mutated = true ; } } fn consume (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn use_cloned (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn fake_read (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
