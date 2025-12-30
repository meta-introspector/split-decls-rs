// Generated macro for impl_885 (impl)
macro_rules! Depcrate_provenance_gcimpl_885 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_885"}
// Dependencies: {}
impl VisitProvenance for StrictPointer { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . provenance . visit_provenance (visit) ; } }
};
}
