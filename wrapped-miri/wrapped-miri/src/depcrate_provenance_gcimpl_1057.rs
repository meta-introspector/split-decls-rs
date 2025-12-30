// Generated macro for impl_1057 (impl)
macro_rules! Depcrate_provenance_gcimpl_1057 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1057"}
// Dependencies: {}
impl VisitProvenance for MPlaceTy < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . ptr () . visit_provenance (visit) ; self . meta () . visit_provenance (visit) ; } }
};
}
