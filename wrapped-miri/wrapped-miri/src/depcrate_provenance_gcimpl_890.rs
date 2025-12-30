// Generated macro for impl_890 (impl)
macro_rules! Depcrate_provenance_gcimpl_890 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_890"}
// Dependencies: {}
impl VisitProvenance for MemPlaceMeta < Provenance > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { MemPlaceMeta :: Meta (m) => m . visit_provenance (visit) , MemPlaceMeta :: None => { } } } }
};
}
