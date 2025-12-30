// Generated macro for impl_839 (impl)
macro_rules! Depcrate_usageimpl_839 {
() => {
// Module: crate::usage
// Provides: {"impl_839"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for BindingUsageFinder < '_ , 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = nested_filter :: OnlyBodies ; fn visit_path (& mut self , path : & hir :: Path < 'tcx > , _ : HirId) -> Self :: Result { if let Res :: Local (id) = path . res && self . binding_ids . contains (& id) { return ControlFlow :: Break (()) ; } ControlFlow :: Continue (()) } fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . cx . tcx } }
};
}
