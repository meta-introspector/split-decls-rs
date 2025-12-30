// Generated macro for WalkEvent (enum)
macro_rules! Depcrate_utility_typesWalkEvent {
() => {
// Module: crate::utility_types
// Provides: {"WalkEvent"}
// Dependencies: {}
# [doc = " `WalkEvent` describes tree walking process."] # [derive (Debug , Copy , Clone)] pub enum WalkEvent < T > { # [doc = " Fired before traversing the node."] Enter (T) , # [doc = " Fired after the node is traversed."] Leave (T) , }
};
}
