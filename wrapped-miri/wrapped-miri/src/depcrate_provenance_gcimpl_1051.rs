// Generated macro for impl_1051 (impl)
macro_rules! Depcrate_provenance_gcimpl_1051 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1051"}
// Dependencies: {}
impl VisitProvenance for Pointer { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . provenance . visit_provenance (visit) ; } }
};
}
