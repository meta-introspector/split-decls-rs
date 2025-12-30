// Generated macro for WalkState (enum)
macro_rules! Depcrate_walkWalkState {
() => {
// Module: crate::walk
// Provides: {"WalkState"}
// Dependencies: {}
# [doc = " WalkState is used in the parallel recursive directory iterator to indicate"] # [doc = " whether walking should continue as normal, skip descending into a"] # [doc = " particular directory or quit the walk entirely."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum WalkState { # [doc = " Continue walking as normal."] Continue , # [doc = " If the directory entry given is a directory, don't descend into it."] # [doc = " In all other cases, this has no effect."] Skip , # [doc = " Quit the entire iterator as soon as possible."] # [doc = ""] # [doc = " Note that this is an inherently asynchronous action. It is possible"] # [doc = " for more entries to be yielded even after instructing the iterator"] # [doc = " to quit."] Quit , }
};
}
