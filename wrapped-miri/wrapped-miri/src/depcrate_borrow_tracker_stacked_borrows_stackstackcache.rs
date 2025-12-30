// Generated macro for StackCache (struct)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_stackStackCache {
() => {
// Module: crate::borrow_tracker::stacked_borrows::stack
// Provides: {"StackCache"}
// Dependencies: {}
# [doc = " A very small cache of searches of a borrow stack, mapping `Item`s to their position in said stack."] # [doc = ""] # [doc = " It may seem like maintaining this cache is a waste for small stacks, but"] # [doc = " (a) iterating over small fixed-size arrays is super fast, and (b) empirically this helps *a lot*,"] # [doc = " probably because runtime is dominated by large stacks."] # [cfg (feature = "stack-cache")] # [derive (Clone , Debug)] struct StackCache { items : [Item ; CACHE_LEN] , idx : [usize ; CACHE_LEN] , }
};
}
