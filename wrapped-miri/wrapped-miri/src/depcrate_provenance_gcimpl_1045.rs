// Generated macro for impl_1045 (impl)
macro_rules! Depcrate_provenance_gcimpl_1045 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1045"}
// Dependencies: {}
impl < A , B > VisitProvenance for (A , B) where A : VisitProvenance , B : VisitProvenance , { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . 0 . visit_provenance (visit) ; self . 1 . visit_provenance (visit) ; } }
};
}
