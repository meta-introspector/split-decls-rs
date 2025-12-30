// Generated macro for impl_896 (impl)
macro_rules! Depcrate_provenance_gcimpl_896 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_896"}
// Dependencies: {}
impl VisitProvenance for crate :: MiriInterpCx < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . memory . alloc_map () . iter (| it | { for (_id , (_kind , alloc)) in it { alloc . visit_provenance (visit) ; } }) ; self . machine . visit_provenance (visit) ; } }
};
}
