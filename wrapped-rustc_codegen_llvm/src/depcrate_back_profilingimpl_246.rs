// Generated macro for impl_246 (impl)
macro_rules! Depcrate_back_profilingimpl_246 {
() => {
// Module: crate::back::profiling
// Provides: {"impl_246"}
// Dependencies: {}
impl < 'a > LlvmSelfProfiler < 'a > { pub (crate) fn new (profiler : Arc < SelfProfiler >) -> Self { let llvm_pass_event_kind = profiler . alloc_string ("LLVM Pass") ; Self { profiler , stack : Vec :: default () , llvm_pass_event_kind } } fn before_pass_callback (& 'a mut self , pass_name : & str , ir_name : & str) { let event_id = llvm_args_to_string_id (& self . profiler , pass_name , ir_name) ; self . stack . push (TimingGuard :: start (& self . profiler , self . llvm_pass_event_kind , event_id)) ; } fn after_pass_callback (& mut self) { self . stack . pop () ; } }
};
}
