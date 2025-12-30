// Generated macro for impl_1054 (impl)
macro_rules! Depcrate_provenance_gcimpl_1054 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1054"}
// Dependencies: {}
impl VisitProvenance for Immediate < Provenance > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { Immediate :: Scalar (s) => { s . visit_provenance (visit) ; } Immediate :: ScalarPair (s1 , s2) => { s1 . visit_provenance (visit) ; s2 . visit_provenance (visit) ; } Immediate :: Uninit => { } } } }
};
}
