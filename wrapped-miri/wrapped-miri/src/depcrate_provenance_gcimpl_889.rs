// Generated macro for impl_889 (impl)
macro_rules! Depcrate_provenance_gcimpl_889 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_889"}
// Dependencies: {}
impl VisitProvenance for Immediate < Provenance > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { Immediate :: Scalar (s) => { s . visit_provenance (visit) ; } Immediate :: ScalarPair (s1 , s2) => { s1 . visit_provenance (visit) ; s2 . visit_provenance (visit) ; } Immediate :: Uninit => { } } } }
};
}
