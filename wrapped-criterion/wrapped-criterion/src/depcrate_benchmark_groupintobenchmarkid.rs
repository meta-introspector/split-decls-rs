// Generated macro for IntoBenchmarkId (trait)
macro_rules! Depcrate_benchmark_groupIntoBenchmarkId {
() => {
// Module: crate::benchmark_group
// Provides: {"IntoBenchmarkId"}
// Dependencies: {}
# [doc = " Sealed trait which allows users to automatically convert strings to benchmark IDs."] pub trait IntoBenchmarkId : private :: Sealed { fn into_benchmark_id (self) -> BenchmarkId ; }
};
}
