// Generated macro for impl_893 (impl)
macro_rules! Depcrate_provenance_gcimpl_893 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_893"}
// Dependencies: {}
impl VisitProvenance for PlaceTy < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self . as_mplace_or_local () { Either :: Left (mplace) => mplace . visit_provenance (visit) , Either :: Right (_) => () , } } }
};
}
