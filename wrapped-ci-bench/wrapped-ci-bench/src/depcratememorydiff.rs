// Generated macro for MemoryDiff (struct)
macro_rules! DepcrateMemoryDiff {
() => {
// Module: crate
// Provides: {"MemoryDiff"}
// Dependencies: {}
# [doc = " Contains information about memory usage and a difference for a specific scenario & comparator"] # [derive (Clone)] struct MemoryDiff { scenario : String , baseline : MemoryDetails , candidate : MemoryDetails , comparator : CompareMemoryOperand , diff : i64 , diff_ratio : f64 , }
};
}
