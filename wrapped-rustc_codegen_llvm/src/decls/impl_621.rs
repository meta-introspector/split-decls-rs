macro_rules! deps {
    () => {
        TimeTraceProfiler!();
    };
}

macro_rules! impl_621 {
    () => {
        deps!();
        impl Drop for TimeTraceProfiler { fn drop (& mut self) { if self . enabled { unsafe { llvm :: LLVMRustTimeTraceProfilerFinishThread () } } } }
    };
}

impl_621!()