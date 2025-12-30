// Generated macro for impl_570 (impl)
macro_rules! Depcrate_concurrency_threadimpl_570 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_570"}
// Dependencies: {}
impl VisitProvenance for ThreadManager < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let ThreadManager { threads , thread_local_allocs , active_thread : _ , yield_active_thread : _ , fixed_scheduling : _ , } = self ; for thread in threads { thread . visit_provenance (visit) ; } for ptr in thread_local_allocs . values () { ptr . visit_provenance (visit) ; } } }
};
}
