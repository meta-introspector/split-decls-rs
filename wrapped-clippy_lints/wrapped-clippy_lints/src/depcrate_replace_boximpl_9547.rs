// Generated macro for impl_9547 (impl)
macro_rules! Depcrate_replace_boximpl_9547 {
() => {
// Module: crate::replace_box
// Provides: {"impl_9547"}
// Dependencies: {}
impl < 'tcx > Delegate < 'tcx > for MovedVariablesCtxt < '_ > { fn consume (& mut self , cmt : & PlaceWithHirId < 'tcx > , _ : HirId) { if let PlaceBase :: Local (id) = cmt . place . base && let mut projections = cmt . place . projections . iter () . filter (| x | matches ! (x . kind , ProjectionKind :: Deref)) && (projections . next () . is_none () || projections . next () . is_some ()) { self . consumed_locals . insert (id) ; } } fn use_cloned (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn borrow (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId , _ : ty :: BorrowKind) { } fn mutate (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn fake_read (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
