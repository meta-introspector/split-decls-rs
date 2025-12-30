// Generated macro for impl_5661 (impl)
macro_rules! Depcrate_methods_iter_overeager_clonedimpl_5661 {
() => {
// Module: crate::methods::iter_overeager_cloned
// Provides: {"impl_5661"}
// Dependencies: {}
impl < 'tcx > Delegate < 'tcx > for MoveDelegate { fn consume (& mut self , place_with_id : & PlaceWithHirId < 'tcx > , _ : HirId) { if let PlaceBase :: Local (l) = place_with_id . place . base { self . used_move . insert (l) ; } } fn use_cloned (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn borrow (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId , _ : BorrowKind) { } fn mutate (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn fake_read (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
