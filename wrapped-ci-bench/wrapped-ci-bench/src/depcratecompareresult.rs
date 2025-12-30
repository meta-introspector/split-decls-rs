// Generated macro for CompareResult (struct)
macro_rules! DepcrateCompareResult {
() => {
// Module: crate
// Provides: {"CompareResult"}
// Dependencies: {}
# [doc = " The results of a comparison between two `run-all` executions"] struct CompareResult { # [doc = " Results for benchmark scenarios we know are fairly deterministic."] # [doc = ""] # [doc = " The string is a detailed diff between the instruction counts obtained from callgrind."] diffs : Vec < (Diff , String) > , # [doc = " Benchmark scenarios present in the candidate but missing in the baseline"] missing_in_baseline : Vec < String > , }
};
}
