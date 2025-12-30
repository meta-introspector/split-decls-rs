// Generated macro for ws_to_crate_graph (function)
macro_rules! Depcrate_reloadws_to_crate_graph {
() => {
// Module: crate::reload
// Provides: {"ws_to_crate_graph"}
// Dependencies: {}
pub fn ws_to_crate_graph (workspaces : & [ProjectWorkspace] , extra_env : & FxHashMap < String , Option < String > > , mut load : impl FnMut (& AbsPath) -> Option < vfs :: FileId > ,) -> (CrateGraphBuilder , Vec < ProcMacroPaths >) { let mut crate_graph = CrateGraphBuilder :: default () ; let mut proc_macro_paths = Vec :: default () ; for ws in workspaces { let (other , mut crate_proc_macros) = ws . to_crate_graph (& mut load , extra_env) ; crate_graph . extend (other , & mut crate_proc_macros) ; proc_macro_paths . push (crate_proc_macros) ; } crate_graph . shrink_to_fit () ; proc_macro_paths . shrink_to_fit () ; (crate_graph , proc_macro_paths) }
};
}
