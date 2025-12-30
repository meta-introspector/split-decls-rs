// Generated macro for BenchmarkStats (struct)
macro_rules! Depcrate_comm_messagesBenchmarkStats {
() => {
// Module: crate::comm::messages
// Provides: {"BenchmarkStats"}
// Dependencies: {}
# [doc = " The stats gathered by a single benchmark execution."] # [doc = " Some of the perf. counters may be missing if the machine that executes the benchmark is unable"] # [doc = " to gather them."] # [derive (Debug , serde :: Serialize , serde :: Deserialize)] pub struct BenchmarkStats { pub cycles : Option < u64 > , pub instructions : Option < u64 > , pub branch_misses : Option < u64 > , pub cache_misses : Option < u64 > , pub cache_references : Option < u64 > , pub wall_time : Duration , }
};
}
