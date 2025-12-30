// Generated macro for impl_887 (impl)
macro_rules! Depcrate_provenance_gcimpl_887 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_887"}
// Dependencies: {}
impl VisitProvenance for Scalar { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { Scalar :: Ptr (ptr , _) => ptr . visit_provenance (visit) , Scalar :: Int (_) => () , } } }
};
}
