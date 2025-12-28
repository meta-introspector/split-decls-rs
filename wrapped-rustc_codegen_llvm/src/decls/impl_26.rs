macro_rules! deps {
    () => {
        TimeTraceProfiler!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl TimeTraceProfiler { fn new (enabled : bool) -> Self { if enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } } TimeTraceProfiler { enabled } } }
    };
}

impl_26!()