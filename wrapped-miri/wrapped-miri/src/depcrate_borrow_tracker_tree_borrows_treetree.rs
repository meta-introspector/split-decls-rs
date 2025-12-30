// Generated macro for Tree (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeTree {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"Tree"}
// Dependencies: {}
# [doc = " Tree structure with both parents and children since we want to be"] # [doc = " able to traverse the tree efficiently in both directions."] # [derive (Clone , Debug)] pub struct Tree { # [doc = " Mapping from tags to keys. The key obtained can then be used in"] # [doc = " any of the `UniValMap` relative to this allocation, i.e. both the"] # [doc = " `nodes` and `rperms` of the same `Tree`."] # [doc = " The parent-child relationship in `Node` is encoded in terms of these same"] # [doc = " keys, so traversing the entire tree needs exactly one access to"] # [doc = " `tag_mapping`."] pub (super) tag_mapping : UniKeyMap < BorTag > , # [doc = " All nodes of this tree."] pub (super) nodes : UniValMap < Node > , # [doc = " Maps a tag and a location to a perm, with possible lazy"] # [doc = " initialization."] # [doc = ""] # [doc = " NOTE: not all tags registered in `nodes` are necessarily in all"] # [doc = " ranges of `rperms`, because `rperms` is in part lazily initialized."] # [doc = " Just because `nodes.get(key)` is `Some(_)` does not mean you can safely"] # [doc = " `unwrap` any `perm.get(key)`."] # [doc = ""] # [doc = " We do uphold the fact that `keys(perms)` is a subset of `keys(nodes)`"] pub (super) rperms : DedupRangeMap < UniValMap < LocationState > > , # [doc = " The index of the root node."] pub (super) root : UniIndex , }
};
}
