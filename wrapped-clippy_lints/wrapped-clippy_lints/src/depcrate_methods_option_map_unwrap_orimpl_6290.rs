// Generated macro for impl_6290 (impl)
macro_rules! Depcrate_methods_option_map_unwrap_orimpl_6290 {
() => {
// Module: crate::methods::option_map_unwrap_or
// Provides: {"impl_6290"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for UnwrapVisitor < '_ , 'tcx > { type NestedFilter = nested_filter :: All ; fn visit_path (& mut self , path : & Path < 'tcx > , _ : HirId) { if let Res :: Local (local_id) = path . res && let Node :: Pat (pat) = self . cx . tcx . hir_node (local_id) && let PatKind :: Binding (_ , local_id , ..) = pat . kind { self . identifiers . insert (local_id) ; } walk_path (self , path) ; } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
