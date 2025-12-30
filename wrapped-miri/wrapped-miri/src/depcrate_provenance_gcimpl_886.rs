// Generated macro for impl_886 (impl)
macro_rules! Depcrate_provenance_gcimpl_886 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_886"}
// Dependencies: {}
impl VisitProvenance for Pointer { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . provenance . visit_provenance (visit) ; } }
};
}
