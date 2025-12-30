// Generated macro for UniValMap (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_unimapUniValMap {
() => {
// Module: crate::borrow_tracker::tree_borrows::unimap
// Provides: {"UniValMap"}
// Dependencies: {}
# [doc = " From UniIndex to V"] # [derive (Debug , Clone , Eq)] pub struct UniValMap < V > { # [doc = " The mapping data. Thanks to Vec we get both fast accesses, and"] # [doc = " a memory-optimal representation if there are few deletions."] data : Vec < Option < V > > , }
};
}
