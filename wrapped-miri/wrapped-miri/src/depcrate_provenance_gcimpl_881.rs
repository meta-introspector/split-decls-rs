// Generated macro for impl_881 (impl)
macro_rules! Depcrate_provenance_gcimpl_881 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_881"}
// Dependencies: {}
impl < T : VisitProvenance > VisitProvenance for std :: cell :: RefCell < T > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . borrow () . visit_provenance (visit) } }
};
}
