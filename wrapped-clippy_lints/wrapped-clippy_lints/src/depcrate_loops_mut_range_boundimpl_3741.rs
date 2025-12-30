// Generated macro for impl_3741 (impl)
macro_rules! Depcrate_loops_mut_range_boundimpl_3741 {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"impl_3741"}
// Dependencies: {}
impl < 'tcx > Delegate < 'tcx > for MutatePairDelegate < '_ , 'tcx > { fn consume (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn use_cloned (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : HirId) { } fn borrow (& mut self , cmt : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId , bk : ty :: BorrowKind) { if bk == ty :: BorrowKind :: Mutable && let PlaceBase :: Local (id) = cmt . place . base { if Some (id) == self . hir_id_low && ! BreakAfterExprVisitor :: is_found (self . cx , diag_expr_id) { self . span_low = Some (self . cx . tcx . hir_span (diag_expr_id)) ; } if Some (id) == self . hir_id_high && ! BreakAfterExprVisitor :: is_found (self . cx , diag_expr_id) { self . span_high = Some (self . cx . tcx . hir_span (diag_expr_id)) ; } } } fn mutate (& mut self , cmt : & PlaceWithHirId < 'tcx > , diag_expr_id : HirId) { if let PlaceBase :: Local (id) = cmt . place . base { if Some (id) == self . hir_id_low && ! BreakAfterExprVisitor :: is_found (self . cx , diag_expr_id) { self . span_low = Some (self . cx . tcx . hir_span (diag_expr_id)) ; } if Some (id) == self . hir_id_high && ! BreakAfterExprVisitor :: is_found (self . cx , diag_expr_id) { self . span_high = Some (self . cx . tcx . hir_span (diag_expr_id)) ; } } } fn fake_read (& mut self , _ : & PlaceWithHirId < 'tcx > , _ : FakeReadCause , _ : HirId) { } }
};
}
