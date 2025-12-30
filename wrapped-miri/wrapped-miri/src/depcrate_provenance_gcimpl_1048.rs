// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_provenance_gcimpl_1048 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1048"}
// Dependencies: {}
impl VisitProvenance for AllocId { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { visit (Some (* self) , None) } }
};
}
