// Generated macro for add_dep_with_prelude (function)
macro_rules! Depcrate_workspaceadd_dep_with_prelude {
() => {
// Module: crate::workspace
// Provides: {"add_dep_with_prelude"}
// Dependencies: {}
fn add_dep_with_prelude (graph : & mut CrateGraphBuilder , from : CrateBuilderId , name : CrateName , to : CrateBuilderId , prelude : bool , sysroot : bool ,) { add_dep_inner (graph , from , DependencyBuilder :: with_prelude (name , to , prelude , sysroot)) }
};
}
