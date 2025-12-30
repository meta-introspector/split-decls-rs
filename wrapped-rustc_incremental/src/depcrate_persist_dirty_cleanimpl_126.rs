// Generated macro for impl_126 (impl)
macro_rules! Depcrate_persist_dirty_cleanimpl_126 {
() => {
// Module: crate::persist::dirty_clean
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'tcx > intravisit :: Visitor < 'tcx > for FindAllAttrs < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_attribute (& mut self , attr : & 'tcx Attribute) { if self . is_active_attr (attr) { self . found_attrs . push (attr) ; } } }
};
}
