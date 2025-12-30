// Generated macro for load_cargo_with_overrides (function)
macro_rules! Depcrate_testsload_cargo_with_overrides {
() => {
// Module: crate::tests
// Provides: {"load_cargo_with_overrides"}
// Dependencies: {}
fn load_cargo_with_overrides (file : & str , cfg_overrides : CfgOverrides ,) -> (CrateGraphBuilder , ProcMacroPaths) { let project_workspace = ProjectWorkspace { cfg_overrides , .. load_workspace_from_metadata (file) } ; to_crate_graph (project_workspace , & mut Default :: default ()) }
};
}
