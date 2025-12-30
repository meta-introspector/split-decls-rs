// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_provenance_gcimpl_1046 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1046"}
// Dependencies: {}
impl < T : VisitProvenance > VisitProvenance for std :: cell :: RefCell < T > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . borrow () . visit_provenance (visit) } }
};
}
