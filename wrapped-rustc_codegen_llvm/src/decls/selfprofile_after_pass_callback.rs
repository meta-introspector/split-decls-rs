macro_rules! deps {
    () => {
        LlvmSelfProfiler!();
    };
}

macro_rules! selfprofile_after_pass_callback {
    () => {
        deps!();
        pub (crate) unsafe extern "C" fn selfprofile_after_pass_callback (llvm_self_profiler : * mut c_void) { let llvm_self_profiler = unsafe { & mut * (llvm_self_profiler as * mut LlvmSelfProfiler < '_ >) } ; llvm_self_profiler . after_pass_callback () ; }
    };
}

selfprofile_after_pass_callback!();