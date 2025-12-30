// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_provenance_gcimpl_1050 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1050"}
// Dependencies: {}
impl VisitProvenance for StrictPointer { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . provenance . visit_provenance (visit) ; } }
};
}
