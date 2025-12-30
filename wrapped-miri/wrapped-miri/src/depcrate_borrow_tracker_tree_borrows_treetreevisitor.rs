// Generated macro for TreeVisitor (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeTreeVisitor {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"TreeVisitor"}
// Dependencies: {}
# [doc = " Internal contents of `Tree` with the minimum of mutable access for"] # [doc = " the purposes of the tree traversal functions: the permissions (`perms`) can be"] # [doc = " updated but not the tree structure (`tag_mapping` and `nodes`)"] struct TreeVisitor < 'tree > { tag_mapping : & 'tree UniKeyMap < BorTag > , nodes : & 'tree mut UniValMap < Node > , perms : & 'tree mut UniValMap < LocationState > , }
};
}
