// Generated macro for impl_748 (impl)
macro_rules! Depcrate_concurrency_genmcimpl_748 {
() => {
// Module: crate::concurrency::genmc
// Provides: {"impl_748"}
// Dependencies: {}
impl VisitProvenance for GenmcCtx { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let genmc_shared_allocs_map = self . exec_state . genmc_shared_allocs_map . borrow () ; for alloc_id in genmc_shared_allocs_map . values () . copied () { visit (Some (alloc_id) , None) ; } } }
};
}
