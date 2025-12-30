// Generated macro for impl_1052 (impl)
macro_rules! Depcrate_provenance_gcimpl_1052 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1052"}
// Dependencies: {}
impl VisitProvenance for Scalar { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { match self { Scalar :: Ptr (ptr , _) => ptr . visit_provenance (visit) , Scalar :: Int (_) => () , } } }
};
}
