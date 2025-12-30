// Generated macro for BenchmarkArgs (struct)
macro_rules! Depcrate_cliBenchmarkArgs {
() => {
// Module: crate::cli
// Provides: {"BenchmarkArgs"}
// Dependencies: {}
# [derive (clap :: Parser , Debug)] pub struct BenchmarkArgs { # [doc = " How many times should each benchmark be repeated."] # [arg (long , default_value = "5")] pub iterations : u32 , # [doc = " Exclude all benchmarks matching a prefix in this comma-separated list"] # [arg (long , value_delimiter = ',')] pub exclude : Vec < String > , # [doc = " Include only benchmarks matching a prefix in this comma-separated list"] # [arg (long , value_delimiter = ',')] pub include : Vec < String > , }
};
}
