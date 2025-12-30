// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_provenance_gcimpl_1058 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1058"}
// Dependencies: {}
impl VisitProvenance for PlaceTy < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self . as_mplace_or_local () { Either :: Left (mplace) => mplace . visit_provenance (visit) , Either :: Right (_) => () , } } }
};
}
