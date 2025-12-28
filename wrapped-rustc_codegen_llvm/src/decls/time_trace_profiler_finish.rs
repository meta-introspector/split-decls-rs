macro_rules! time_trace_profiler_finish {
    () => {
        pub (crate) fn time_trace_profiler_finish (file_name : & Path) { unsafe { let file_name = path_to_c_string (file_name) ; llvm :: LLVMRustTimeTraceProfilerFinish (file_name . as_ptr ()) ; } }
    };
}

time_trace_profiler_finish!()