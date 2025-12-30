// Generated macro for impl_518 (impl)
macro_rules! Depcrate_concurrency_threadimpl_518 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_518"}
// Dependencies: {}
impl VisitProvenance for Thread < '_ > { fn visit_provenance (& self , visit : & mut VisitWith < '_ >) { let Thread { unwind_payloads : panic_payload , last_error , stack , top_user_relevant_frame : _ , state : _ , thread_name : _ , join_status : _ , on_stack_empty : _ , } = self ; for payload in panic_payload { payload . visit_provenance (visit) ; } last_error . visit_provenance (visit) ; for frame in stack { frame . visit_provenance (visit) } } }
};
}
