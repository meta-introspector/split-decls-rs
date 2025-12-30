// Generated macro for selfprofile_after_pass_callback (function)
macro_rules! Depcrate_back_profilingselfprofile_after_pass_callback {
() => {
// Module: crate::back::profiling
// Provides: {"selfprofile_after_pass_callback"}
// Dependencies: {}
pub (crate) unsafe extern "C" fn selfprofile_after_pass_callback (llvm_self_profiler : * mut c_void) { let llvm_self_profiler = unsafe { & mut * (llvm_self_profiler as * mut LlvmSelfProfiler < '_ >) } ; llvm_self_profiler . after_pass_callback () ; }
};
}
