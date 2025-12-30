// Generated macro for add_dep (function)
macro_rules! Depcrate_workspaceadd_dep {
() => {
// Module: crate::workspace
// Provides: {"add_dep"}
// Dependencies: {}
fn add_dep (graph : & mut CrateGraphBuilder , from : CrateBuilderId , name : CrateName , to : CrateBuilderId ,) { add_dep_inner (graph , from , DependencyBuilder :: new (name , to)) }
};
}
