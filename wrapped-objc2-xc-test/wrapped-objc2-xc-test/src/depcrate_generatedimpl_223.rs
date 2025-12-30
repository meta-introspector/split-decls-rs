// Generated macro for impl_223 (impl)
macro_rules! Depcrate_generatedimpl_223 {
() => {
// Module: crate::generated
// Provides: {"impl_223"}
// Dependencies: {}
impl XCTCPUMetric { extern_methods ! (# [doc = " Creates a metric which will observe CPU activity for the thread that executes"] # [doc = " the block being measured. For single-threaded workloads, this provides greater"] # [doc = " precision and lower variance than -init."] # [doc = ""] # [doc = ""] # [doc = " Note: The Thread under test is defined as the thread which will perform the execution of the work provided to the -[XCTestCase measure*] API."] # [doc = ""] # [doc = ""] # [doc = " Returns: An initialized metric."] # [unsafe (method (initLimitingToCurrentThread :))] # [unsafe (method_family = init)] pub fn initLimitingToCurrentThread (this : Allocated < Self >, limit_to_current_thread : bool ,) -> Retained < Self >; # [doc = " Creates a metric that will target the current process."] # [doc = ""] # [doc = ""] # [doc = " Returns: An initialized metric."] # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >;) ; }
};
}
