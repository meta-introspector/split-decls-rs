// Generated macro for Event (enum)
macro_rules! Depcrate_traversalsEvent {
() => {
// Module: crate::traversals
// Provides: {"Event"}
// Dependencies: {}
# [doc = " A low-level DFS traversal event: either entering or exiting the traversal of"] # [doc = " a block."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum Event { # [doc = " Entering traversal of a block."] # [doc = ""] # [doc = " Processing a block upon this event corresponds to a pre-order,"] # [doc = " depth-first traversal."] Enter , # [doc = " Exiting traversal of a block."] # [doc = ""] # [doc = " Processing a block upon this event corresponds to a post-order,"] # [doc = " depth-first traversal."] Exit , }
};
}
