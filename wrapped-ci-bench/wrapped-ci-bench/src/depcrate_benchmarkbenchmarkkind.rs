// Generated macro for BenchmarkKind (enum)
macro_rules! Depcrate_benchmarkBenchmarkKind {
() => {
// Module: crate::benchmark
// Provides: {"BenchmarkKind"}
// Dependencies: {}
# [doc = " Specifies which functionality is being benchmarked"] # [derive (Copy , Clone)] pub enum BenchmarkKind { # [doc = " Perform the handshake and exit"] Handshake (ResumptionKind) , # [doc = " Perform the handshake and transfer 1MB of data"] Transfer , }
};
}
