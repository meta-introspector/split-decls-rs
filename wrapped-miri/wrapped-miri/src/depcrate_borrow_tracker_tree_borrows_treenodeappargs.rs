// Generated macro for NodeAppArgs (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrows_treeNodeAppArgs {
() => {
// Module: crate::borrow_tracker::tree_borrows::tree
// Provides: {"NodeAppArgs"}
// Dependencies: {}
# [doc = " Data given to the transition function"] struct NodeAppArgs < 'node > { # [doc = " Node on which the transition is currently being applied"] node : & 'node mut Node , # [doc = " Mutable access to its permissions"] perm : UniEntry < 'node , LocationState > , # [doc = " Relative position of the access"] rel_pos : AccessRelatedness , }
};
}
