// Generated macro for impl_884 (impl)
macro_rules! Depcrate_provenance_gcimpl_884 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_884"}
// Dependencies: {}
impl VisitProvenance for Provenance { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { if let Provenance :: Concrete { alloc_id , tag , .. } = self { visit (Some (* alloc_id) , Some (* tag)) ; } } }
};
}
