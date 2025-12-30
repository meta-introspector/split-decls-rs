// Generated macro for add_benchmark_group (function)
macro_rules! Depcrateadd_benchmark_group {
() => {
// Module: crate
// Provides: {"add_benchmark_group"}
// Dependencies: {}
# [doc = " Adds a group of benchmarks for the specified parameters"] # [doc = ""] # [doc = " The benchmarks in the group are:"] # [doc = ""] # [doc = " - Handshake without resumption"] # [doc = " - Handshake with session id resumption"] # [doc = " - Handshake with ticket resumption"] # [doc = " - Transfer a 1MB data stream from the server to the client"] fn add_benchmark_group (benchmarks : & mut Vec < Benchmark > , params : BenchmarkParams) { let params_label = params . label . clone () ; for & resumption_param in ResumptionKind :: ALL { let handshake_bench = Benchmark :: new (format ! ("handshake_{}_{params_label}" , resumption_param . label ()) , BenchmarkKind :: Handshake (resumption_param) , params . clone () ,) ; benchmarks . push (handshake_bench) ; } benchmarks . push (Benchmark :: new (format ! ("transfer_no_resume_{params_label}") , BenchmarkKind :: Transfer , params . clone () ,)) ; }
};
}
