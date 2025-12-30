// Generated macro for impl_879 (impl)
macro_rules! Depcrate_provenance_gcimpl_879 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_879"}
// Dependencies: {}
impl < T : VisitProvenance > VisitProvenance for Option < T > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { if let Some (x) = self { x . visit_provenance (visit) ; } } }
};
}
