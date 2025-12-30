// Generated macro for RemoveAction (enum)
macro_rules! Depcrate_nodes_btreeRemoveAction {
() => {
// Module: crate::nodes::btree
// Provides: {"RemoveAction"}
// Dependencies: {}
enum RemoveAction { DeleteAt (usize) , PullUp (Boundary , usize , usize) , Merge (usize) , StealFromLeft (usize) , StealFromRight (usize) , MergeFirst (usize) , ContinueDown (usize) , }
};
}
