// Generated macro for BenchmarkId (struct)
macro_rules! Depcrate_benchmark_groupBenchmarkId {
() => {
// Module: crate::benchmark_group
// Provides: {"BenchmarkId"}
// Dependencies: {}
# [doc = " Simple structure representing an ID for a benchmark. The ID must be unique within a benchmark"] # [doc = " group."] # [derive (Clone , Eq , PartialEq , Hash)] pub struct BenchmarkId { pub (crate) function_name : Option < String > , pub (crate) parameter : Option < String > , }
};
}
