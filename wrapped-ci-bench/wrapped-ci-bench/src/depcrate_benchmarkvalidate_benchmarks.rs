// Generated macro for validate_benchmarks (function)
macro_rules! Depcrate_benchmarkvalidate_benchmarks {
() => {
// Module: crate::benchmark
// Provides: {"validate_benchmarks"}
// Dependencies: {}
# [doc = " Validates a benchmark collection, returning an error if the provided benchmarks are invalid"] # [doc = ""] # [doc = " Benchmarks can be invalid because of the following reasons:"] # [doc = ""] # [doc = " - Re-using an already defined benchmark name."] pub (crate) fn validate_benchmarks (benchmarks : & [Benchmark]) -> anyhow :: Result < () > { let duplicate_names : Vec < _ > = benchmarks . iter () . map (| b | b . name . as_str ()) . duplicates () . collect () ; if ! duplicate_names . is_empty () { anyhow :: bail ! ("The following benchmarks are defined multiple times: {}" , duplicate_names . join (", ")) ; } Ok (()) }
};
}
