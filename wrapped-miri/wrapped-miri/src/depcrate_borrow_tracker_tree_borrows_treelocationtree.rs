// Generated macro for LocationTree (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeLocationTree {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"LocationTree"}
// Dependencies: {}
# [doc = " The state of the full tree for a particular location: for all nodes, the local permissions"] # [doc = " of that node, and the tracking for wildcard accesses."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct LocationTree { # [doc = " Maps a tag to a perm, with possible lazy initialization."] # [doc = ""] # [doc = " NOTE: not all tags registered in `Tree::nodes` are necessarily in all"] # [doc = " ranges of `perms`, because `perms` is in part lazily initialized."] # [doc = " Just because `nodes.get(key)` is `Some(_)` does not mean you can safely"] # [doc = " `unwrap` any `perm.get(key)`."] # [doc = ""] # [doc = " We do uphold the fact that `keys(perms)` is a subset of `keys(nodes)`"] pub perms : UniValMap < LocationState > , # [doc = " Maps a tag and a location to its wildcard access tracking information,"] # [doc = " with possible lazy initialization."] # [doc = ""] # [doc = " If this allocation doesn't have any exposed nodes, then this map doesn't get"] # [doc = " initialized. This way we only need to allocate the map if we need it."] # [doc = ""] # [doc = " NOTE: same guarantees on entry initialization as for `perms`."] pub wildcard_accesses : UniValMap < WildcardState > , }
};
}
