// Generated macro for TreeWalkResult (enum)
macro_rules! Depcrate_treeTreeWalkResult {
() => {
// Module: crate::tree
// Provides: {"TreeWalkResult"}
// Dependencies: {}
# [doc = " Possible return codes for tree walking callback functions."] # [repr (i32)] pub enum TreeWalkResult { # [doc = " Continue with the traversal as normal."] Ok = 0 , # [doc = " Skip the current node (in pre-order mode)."] Skip = 1 , # [doc = " Completely stop the traversal."] Abort = raw :: GIT_EUSER , }
};
}
