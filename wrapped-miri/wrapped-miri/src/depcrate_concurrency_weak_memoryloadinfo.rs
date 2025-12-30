// Generated macro for LoadInfo (struct)
macro_rules! Depcrate_concurrency_weak_memoryLoadInfo {
() => {
// Module: crate::concurrency::weak_memory
// Provides: {"LoadInfo"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Default)] struct LoadInfo { # [doc = " Timestamp of first loads from this store element by each thread"] timestamps : FxHashMap < VectorIdx , VTimestamp > , # [doc = " Whether this store element has been read by an SC load"] sc_loaded : bool , }
};
}
