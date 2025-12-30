// Generated macro for impl_6050 (impl)
macro_rules! Depcrate_methods_needless_collectimpl_6050 {
() => {
// Module: crate::methods::needless_collect
// Provides: {"impl_6050"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for UsedCountVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: OnlyBodies ; fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) { if path_to_local_id (expr , self . id) { self . count += 1 ; } else { walk_expr (self , expr) ; } } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
