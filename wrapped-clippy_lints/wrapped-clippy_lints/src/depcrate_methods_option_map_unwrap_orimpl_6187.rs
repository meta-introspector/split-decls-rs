// Generated macro for impl_6187 (impl)
macro_rules! Depcrate_methods_option_map_unwrap_orimpl_6187 {
() => {
// Module: crate::methods::option_map_unwrap_or
// Provides: {"impl_6187"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ReferenceVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: All ; type Result = ControlFlow < () > ; fn visit_expr (& mut self , expr : & 'tcx rustc_hir :: Expr < '_ >) -> ControlFlow < () > { if expr . span < self . unwrap_or_span && let ExprKind :: Path (ref path) = expr . kind && let QPath :: Resolved (_ , path) = path && let Res :: Local (local_id) = path . res && let Node :: Pat (pat) = self . cx . tcx . hir_node (local_id) && let PatKind :: Binding (_ , local_id , ..) = pat . kind && self . identifiers . contains (& local_id) { return ControlFlow :: Break (()) ; } rustc_hir :: intravisit :: walk_expr (self , expr) } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
