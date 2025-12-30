// Generated macro for impl_527 (impl)
macro_rules! Depcrate_fn_ctxt_checksimpl_527 {
() => {
// Module: crate::fn_ctxt::checks
// Provides: {"impl_527"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for FindClosureArg < 'tcx > { type NestedFilter = rustc_middle :: hir :: nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_expr (& mut self , ex : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Call (rcvr , args) = ex . kind { self . calls . push ((rcvr , args)) ; } hir :: intravisit :: walk_expr (self , ex) ; } }
};
}
