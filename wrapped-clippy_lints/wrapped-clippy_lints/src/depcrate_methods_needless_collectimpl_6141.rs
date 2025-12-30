// Generated macro for impl_6141 (impl)
macro_rules! Depcrate_methods_needless_collectimpl_6141 {
() => {
// Module: crate::methods::needless_collect
// Provides: {"impl_6141"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for UsedCountVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { if expr . res_local_id () == Some (self . id) { self . count += 1 ; } else { walk_expr (self , expr) ; } } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
