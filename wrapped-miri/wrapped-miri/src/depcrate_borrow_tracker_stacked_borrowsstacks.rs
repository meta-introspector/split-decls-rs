// Generated macro for Stacks (struct)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsStacks {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"Stacks"}
// Dependencies: {}
# [doc = " Extra per-allocation state."] # [derive (Clone , Debug)] pub struct Stacks { stacks : DedupRangeMap < Stack > , # [doc = " Stores past operations on this allocation"] history : AllocHistory , # [doc = " The set of tags that have been exposed inside this allocation."] exposed_tags : FxHashSet < BorTag > , }
};
}
