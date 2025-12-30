// Generated macro for Args (enum)
macro_rules! Depcrate_cliArgs {
() => {
// Module: crate::cli
// Provides: {"Args"}
// Dependencies: {}
# [derive (clap :: Parser , Debug)] pub enum Args { # [doc = " Benchmark all benchmarks in this benchmark group and print the results as JSON."] Run (BenchmarkArgs) , # [doc = " Profile a single benchmark execution."] Profile (ProfileArgs) , # [doc = " List benchmarks that are defined in the current group as a JSON array."] List , }
};
}
