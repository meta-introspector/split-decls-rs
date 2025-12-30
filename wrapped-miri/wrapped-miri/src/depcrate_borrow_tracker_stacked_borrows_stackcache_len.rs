// Generated macro for CACHE_LEN (const)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_stackCACHE_LEN {
() => {
// Module: crate::borrow_tracker::stacked_borrows::stack
// Provides: {"CACHE_LEN"}
// Dependencies: {}
# [doc = " Exactly what cache size we should use is a difficult trade-off. There will always be some"] # [doc = " workload which has a `BorTag` working set which exceeds the size of the cache, and ends up"] # [doc = " falling back to linear searches of the borrow stack very often."] # [doc = " The cost of making this value too large is that the loop in `Stack::insert` which ensures the"] # [doc = " entries in the cache stay correct after an insert becomes expensive."] # [cfg (feature = "stack-cache")] const CACHE_LEN : usize = 32 ;
};
}
