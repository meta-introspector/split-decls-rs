// Generated macro for erase_cached_source_time (function)
macro_rules! Depcrateerase_cached_source_time {
() => {
// Module: crate
// Provides: {"erase_cached_source_time"}
// Dependencies: {}
# [doc = " Erases the cached source timestamp.  Public for tests."] # [doc (hidden)] pub fn erase_cached_source_time () { SOURCE_EPOCH_SECONDS . store (u64 :: MAX , Ordering :: Release) ; }
};
}
