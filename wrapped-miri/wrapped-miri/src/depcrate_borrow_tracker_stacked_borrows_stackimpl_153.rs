// Generated macro for impl_153 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_stackimpl_153 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::stack
// Provides: {"impl_153"}
// Dependencies: {}
# [cfg (feature = "stack-cache")] impl StackCache { # [doc = " When a tag is used, we call this function to add or refresh it in the cache."] # [doc = ""] # [doc = " We use the position in the cache to represent how recently a tag was used; the first position"] # [doc = " is the most recently used tag. So an add shifts every element towards the end, and inserts"] # [doc = " the new element at the start. We lose the last element."] # [doc = " This strategy is effective at keeping the most-accessed items in the cache, but it costs a"] # [doc = " linear shift across the entire cache when we add a new tag."] fn add (& mut self , idx : usize , item : Item) { self . items . copy_within (0 .. CACHE_LEN - 1 , 1) ; self . items [0] = item ; self . idx . copy_within (0 .. CACHE_LEN - 1 , 1) ; self . idx [0] = idx ; } }
};
}
