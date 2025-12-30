// Generated macro for UniKeyMap (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_unimapUniKeyMap {
() => {
// Module: crate::borrow_tracker::tree_borrows::unimap
// Provides: {"UniKeyMap"}
// Dependencies: {}
# [doc = " From K to UniIndex"] # [derive (Debug , Clone , Default)] pub struct UniKeyMap < K > { # [doc = " Underlying map that does all the hard work."] # [doc = " Key invariant: the contents of `deassigned` are disjoint from the"] # [doc = " keys of `mapping`, and together they form the set of contiguous integers"] # [doc = " `0 .. (mapping.len() + deassigned.len())`."] mapping : FxHashMap < K , u32 > , # [doc = " Indexes that can be reused: memory gain when the map gets sparse"] # [doc = " due to many deletions."] deassigned : Vec < u32 > , }
};
}
