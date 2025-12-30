// Generated macro for MemoryCompareResult (struct)
macro_rules! DepcrateMemoryCompareResult {
() => {
// Module: crate
// Provides: {"MemoryCompareResult"}
// Dependencies: {}
# [doc = " The results of a comparison between two `run-all` executions"] struct MemoryCompareResult { diffs : Vec < MemoryDiff > , # [doc = " Benchmark scenarios present in the candidate but missing in the baseline"] missing_in_baseline : Vec < String > , }
};
}
