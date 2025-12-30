// Generated macro for impl_517 (impl)
macro_rules! Depcrate_concurrency_threadimpl_517 {
() => {
// Module: crate::concurrency::thread
// Provides: {"impl_517"}
// Dependencies: {}
impl < 'tcx > Thread < 'tcx > { fn new (name : Option < & str > , on_stack_empty : Option < StackEmptyCallback < 'tcx > >) -> Self { Self { state : ThreadState :: Enabled , thread_name : name . map (| name | Vec :: from (name . as_bytes ())) , stack : Vec :: new () , top_user_relevant_frame : None , join_status : ThreadJoinStatus :: Joinable , unwind_payloads : Vec :: new () , last_error : None , on_stack_empty , } } }
};
}
