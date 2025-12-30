// Generated macro for impl_892 (impl)
macro_rules! Depcrate_provenance_gcimpl_892 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_892"}
// Dependencies: {}
impl VisitProvenance for MPlaceTy < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . ptr () . visit_provenance (visit) ; self . meta () . visit_provenance (visit) ; } }
};
}
