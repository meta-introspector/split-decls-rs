// Generated macro for get_reported_instr_count (function)
macro_rules! Depcrate_benchmarkget_reported_instr_count {
() => {
// Module: crate::benchmark
// Provides: {"get_reported_instr_count"}
// Dependencies: {}
# [doc = " Get the reported instruction counts for the provided benchmark"] pub (crate) fn get_reported_instr_count (bench : & Benchmark , results : & FxHashMap < & str , InstructionCounts > ,) -> InstructionCounts { results [& bench . name ()] }
};
}
