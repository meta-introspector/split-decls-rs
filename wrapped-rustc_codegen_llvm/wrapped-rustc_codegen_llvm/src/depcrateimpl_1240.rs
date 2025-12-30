// Generated macro for impl_1240 (impl)
macro_rules! Depcrateimpl_1240 {
() => {
// Module: crate
// Provides: {"impl_1240"}
// Dependencies: {}
impl Drop for TimeTraceProfiler { fn drop (& mut self) { if self . enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerFinishThread () } } } }
};
}
