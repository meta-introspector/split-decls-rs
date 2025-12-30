// Generated macro for BenchmarkResult (struct)
macro_rules! Depcrate_comm_messagesBenchmarkResult {
() => {
// Module: crate::comm::messages
// Provides: {"BenchmarkResult"}
// Dependencies: {}
# [doc = " Stats gathered by several iterations of a single benchmark."] # [derive (Debug , serde :: Serialize , serde :: Deserialize)] pub struct BenchmarkResult { pub name : String , pub stats : Vec < BenchmarkStats > , }
};
}
