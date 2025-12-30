// Generated macro for impl_8006 (impl)
macro_rules! Depcrate_needless_pass_by_valueimpl_8006 {
() => {
// Module: crate::needless_pass_by_value
// Provides: {"impl_8006"}
// Dependencies: {}
impl < 'tcx > euv :: Delegate < 'tcx > for MovedVariablesCtxt { fn consume (& mut self , cmt : & euv :: PlaceWithHirId < 'tcx > , _ : HirId) { self . move_common (cmt) ; } fn use_cloned (& mut self , _ : & euv :: PlaceWithHirId < 'tcx > , _ : HirId) { } fn borrow (& mut self , _ : & euv :: PlaceWithHirId < 'tcx > , _ : HirId , _ : ty :: BorrowKind) { } fn mutate (& mut self , _ : & euv :: PlaceWithHirId < 'tcx > , _ : HirId) { } fn fake_read (& mut self , _ : & rustc_hir_typeck :: expr_use_visitor :: PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
