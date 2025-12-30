// Generated macro for impl_8801 (impl)
macro_rules! Depcrate_option_if_let_elseimpl_8801 {
() => {
// Module: crate::option_if_let_else
// Provides: {"impl_8801"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ReferenceVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: All ; type Result = ControlFlow < () > ; fn visit_expr (& mut self , expr : & 'tcx Expr < '_ >) -> ControlFlow < () > { if let ExprKind :: Path (ref path) = expr . kind && let QPath :: Resolved (_ , path) = path && let Res :: Local (local_id) = path . res && let Node :: Pat (pat) = self . cx . tcx . hir_node (local_id) && let PatKind :: Binding (_ , local_id , ..) = pat . kind && self . identifiers . contains (& local_id) { return ControlFlow :: Break (()) ; } walk_expr (self , expr) } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
