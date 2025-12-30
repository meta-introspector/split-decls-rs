// Generated macro for impl_895 (impl)
macro_rules! Depcrate_provenance_gcimpl_895 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_895"}
// Dependencies: {}
impl VisitProvenance for Allocation < Provenance , AllocExtra < '_ > , MiriAllocBytes > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { for prov in self . provenance () . provenances () { prov . visit_provenance (visit) ; } self . extra . visit_provenance (visit) ; } }
};
}
