// Generated macro for load_cargo (function)
macro_rules! Depcrate_testsload_cargo {
() => {
// Module: crate::tests
// Provides: {"load_cargo"}
// Dependencies: {}
fn load_cargo (file : & str) -> (CrateGraphBuilder , ProcMacroPaths) { let project_workspace = load_workspace_from_metadata (file) ; to_crate_graph (project_workspace , & mut Default :: default ()) }
};
}
