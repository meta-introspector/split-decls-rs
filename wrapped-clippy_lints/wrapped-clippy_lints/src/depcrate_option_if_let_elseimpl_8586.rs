// Generated macro for impl_8586 (impl)
macro_rules! Depcrate_option_if_let_elseimpl_8586 {
() => {
// Module: crate::option_if_let_else
// Provides: {"impl_8586"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ConditionVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: All ; fn visit_path (& mut self , path : & Path < 'tcx > , _ : HirId) { if let Res :: Local (local_id) = path . res && let Node :: Pat (pat) = self . cx . tcx . hir_node (local_id) && let PatKind :: Binding (_ , local_id , ..) = pat . kind { self . identifiers . insert (local_id) ; } walk_path (self , path) ; } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
