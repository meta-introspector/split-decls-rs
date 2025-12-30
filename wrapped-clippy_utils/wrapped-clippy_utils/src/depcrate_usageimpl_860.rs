// Generated macro for impl_860 (impl)
macro_rules! Depcrate_usageimpl_860 {
() => {
// Module: crate::usage
// Provides: {"impl_860"}
// Dependencies: {}
impl < 'tcx > Delegate < 'tcx > for MutVarsDelegate { fn consume (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn use_cloned (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn borrow (& mut self , cmt : & PlaceWithHirId < 'tcx > , _ : HirId , bk : ty :: BorrowKind) { if bk == ty :: BorrowKind :: Mutable { self . update (cmt) ; } } fn mutate (& mut self , cmt : & PlaceWithHirId < 'tcx > , _ : HirId) { self . update (cmt) ; } fn fake_read (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
