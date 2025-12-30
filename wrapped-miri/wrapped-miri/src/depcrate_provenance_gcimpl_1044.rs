// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_provenance_gcimpl_1044 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1044"}
// Dependencies: {}
impl < T : VisitProvenance > VisitProvenance for Option < T > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { if let Some (x) = self { x . visit_provenance (visit) ; } } }
};
}
