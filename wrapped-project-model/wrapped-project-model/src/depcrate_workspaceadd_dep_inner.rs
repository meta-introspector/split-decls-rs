// Generated macro for add_dep_inner (function)
macro_rules! Depcrate_workspaceadd_dep_inner {
() => {
// Module: crate::workspace
// Provides: {"add_dep_inner"}
// Dependencies: {}
fn add_dep_inner (graph : & mut CrateGraphBuilder , from : CrateBuilderId , dep : DependencyBuilder) { if let Err (err) = graph . add_dep (from , dep) { tracing :: warn ! ("{}" , err) } }
};
}
