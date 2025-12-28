macro_rules! deps {
    () => {
        TimeTraceProfiler!();
    };
}

macro_rules! impl_620 {
    () => {
        deps!();
        impl TimeTraceProfiler { fn new (enabled : bool) -> Self { if enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerInitialize () } } TimeTraceProfiler { enabled } } }
    };
}

impl_620!()