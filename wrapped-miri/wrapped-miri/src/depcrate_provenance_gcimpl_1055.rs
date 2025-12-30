// Generated macro for impl_1055 (impl)
macro_rules! Depcrate_provenance_gcimpl_1055 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1055"}
// Dependencies: {}
impl VisitProvenance for MemPlaceMeta < Provenance > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { MemPlaceMeta :: Meta (m) => m . visit_provenance (visit) , MemPlaceMeta :: None => { } } } }
};
}
