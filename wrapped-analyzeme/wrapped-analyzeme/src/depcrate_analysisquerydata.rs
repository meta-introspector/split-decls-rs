// Generated macro for QueryData (struct)
macro_rules! Depcrate_analysisQueryData {
() => {
// Module: crate::analysis
// Provides: {"QueryData"}
// Dependencies: {}
# [doc = " Data related to profiling a specific rustc query"] # [derive (Serialize , Deserialize , Clone , Debug , Default)] pub struct QueryData { pub label : String , pub time : Duration , pub self_time : Duration , pub number_of_cache_misses : usize , pub number_of_cache_hits : usize , # [doc = " How many times was the query/event actually executed (without a cache hit)."] # [doc = " Note that for queries, this should correspond to `number_of_cache_misses`, however"] # [doc = " for other types of activities we don't actually count cache misses."] pub invocation_count : usize , pub blocked_time : Duration , pub incremental_load_time : Duration , pub incremental_hashing_time : Duration , }
};
}
