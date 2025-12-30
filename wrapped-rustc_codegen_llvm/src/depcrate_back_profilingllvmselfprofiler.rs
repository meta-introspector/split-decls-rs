// Generated macro for LlvmSelfProfiler (struct)
macro_rules! Depcrate_back_profilingLlvmSelfProfiler {
() => {
// Module: crate::back::profiling
// Provides: {"LlvmSelfProfiler"}
// Dependencies: {}
pub (crate) struct LlvmSelfProfiler < 'a > { profiler : Arc < SelfProfiler > , stack : Vec < TimingGuard < 'a > > , llvm_pass_event_kind : StringId , }
};
}
