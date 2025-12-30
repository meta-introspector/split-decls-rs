// Generated macro for impl_68 (impl)
macro_rules! Depcrate_benchmark_groupimpl_68 {
() => {
// Module: crate::benchmark_group
// Provides: {"impl_68"}
// Dependencies: {}
impl < S : Into < String > > IntoBenchmarkId for S { fn into_benchmark_id (self) -> BenchmarkId { let function_name = self . into () ; assert ! (! function_name . is_empty () , "Function name must not be empty.") ; BenchmarkId { function_name : Some (function_name) , parameter : None , } } }
};
}
