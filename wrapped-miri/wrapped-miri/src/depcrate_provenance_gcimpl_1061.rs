// Generated macro for impl_1061 (impl)
macro_rules! Depcrate_provenance_gcimpl_1061 {
() => {
// Module: crate::provenance_gc
// Provides: {"impl_1061"}
// Dependencies: {}
impl VisitProvenance for crate :: MiriInterpCx < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { self . memory . alloc_map () . iter (| it | { for (_id , (_kind , alloc)) in it { alloc . visit_provenance (visit) ; } }) ; self . machine . visit_provenance (visit) ; } }
};
}
