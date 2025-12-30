// Generated macro for impl_1239 (impl)
macro_rules! Depcrateimpl_1239 {
() => {
// Module: crate
// Provides: {"impl_1239"}
// Dependencies: {}
impl TimeTraceProfiler { fn new (enabled : bool) -> Self { if enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } } TimeTraceProfiler { enabled } } }
};
}
