// Generated macro for time_trace_profiler_finish (function)
macro_rules! Depcrate_llvm_utiltime_trace_profiler_finish {
() => {
// Module: crate::llvm_util
// Provides: {"time_trace_profiler_finish"}
// Dependencies: {}
pub (crate) fn time_trace_profiler_finish (file_name : & Path) { unsafe { let file_name = path_to_c_string (file_name) ; llvm :: LLVMRustTimeTraceProfilerFinish (file_name . as_ptr ()) ; } }
};
}
