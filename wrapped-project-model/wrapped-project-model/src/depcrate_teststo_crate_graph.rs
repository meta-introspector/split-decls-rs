// Generated macro for to_crate_graph (function)
macro_rules! Depcrate_teststo_crate_graph {
() => {
// Module: crate::tests
// Provides: {"to_crate_graph"}
// Dependencies: {}
fn to_crate_graph (project_workspace : ProjectWorkspace , file_map : & mut FxHashMap < AbsPathBuf , FileId > ,) -> (CrateGraphBuilder , ProcMacroPaths) { project_workspace . to_crate_graph (& mut { | path | { let len = file_map . len () + 1 ; Some (* file_map . entry (path . to_path_buf ()) . or_insert (FileId :: from_raw (len as u32))) } } , & Default :: default () ,) }
};
}
