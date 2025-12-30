// Generated macro for RecursionStep (enum)
macro_rules! Depcrate_algo_articulation_pointsRecursionStep {
() => {
// Module: crate::algo::articulation_points
// Provides: {"RecursionStep"}
// Dependencies: {}
# [doc = " Small helper enum that defines the various splitup recursion steps of Tarjan's algorithm."] enum RecursionStep { BaseStep (usize) , ProcessChildStep (usize , usize) , NoBackEdgeConditionCheck (usize , usize) , RootMoreThanTwoChildrenCheck (usize) , }
};
}
