// Generated macro for ThreadIdMap (struct)
macro_rules! Depcrate_concurrency_genmc_thread_id_mapThreadIdMap {
() => {
// Module: crate::concurrency::genmc::thread_id_map
// Provides: {"ThreadIdMap"}
// Dependencies: {}
# [derive (Debug)] pub struct ThreadIdMap { # [doc = " Map from Miri thread IDs to GenMC thread IDs."] # [doc = " We assume as little as possible about Miri thread IDs, so we use a map."] miri_to_genmc : FxHashMap < ThreadId , i32 > , # [doc = " Map from GenMC thread IDs to Miri thread IDs."] # [doc = " We control which thread IDs are used, so we choose them in as an incrementing counter."] genmc_to_miri : Vec < ThreadId > , }
};
}
