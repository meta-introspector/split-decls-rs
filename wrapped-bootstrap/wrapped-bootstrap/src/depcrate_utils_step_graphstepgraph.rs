// Generated macro for StepGraph (struct)
macro_rules! Depcrate_utils_step_graphStepGraph {
() => {
// Module: crate::utils::step_graph
// Provides: {"StepGraph"}
// Dependencies: {}
# [doc = " Records the executed steps and their dependencies in a directed graph,"] # [doc = " which can then be rendered into a DOT file for visualization."] # [doc = ""] # [doc = " The graph visualizes the first execution of a step with a solid edge,"] # [doc = " and cached executions of steps with a dashed edge."] # [doc = " If you only want to see first executions, you can modify the code in `DotGraph` to"] # [doc = " always set `cached: false`."] # [derive (Default)] pub struct StepGraph { # [doc = " We essentially store one graph per dry run mode."] graphs : HashMap < String , DotGraph > , }
};
}
