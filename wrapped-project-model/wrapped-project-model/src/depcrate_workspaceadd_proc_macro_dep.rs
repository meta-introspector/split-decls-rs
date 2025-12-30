// Generated macro for add_proc_macro_dep (function)
macro_rules! Depcrate_workspaceadd_proc_macro_dep {
() => {
// Module: crate::workspace
// Provides: {"add_proc_macro_dep"}
// Dependencies: {}
fn add_proc_macro_dep (crate_graph : & mut CrateGraphBuilder , from : CrateBuilderId , to : CrateBuilderId , prelude : bool ,) { add_dep_with_prelude (crate_graph , from , CrateName :: new ("proc_macro") . unwrap () , to , prelude , true ,) ; }
};
}
