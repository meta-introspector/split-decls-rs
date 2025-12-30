// Generated macro for test_get_cargo_workspace_manifest_dir (function)
macro_rules! Depcrate_envtest_get_cargo_workspace_manifest_dir {
() => {
// Module: crate::env
// Provides: {"test_get_cargo_workspace_manifest_dir"}
// Dependencies: {}
# [test] fn test_get_cargo_workspace_manifest_dir () { let workspace = get_cargo_workspace (Workspace :: DetectWithCargo (env ! ("CARGO_MANIFEST_DIR"))) ; let manifest_dir = PathBuf :: from (env ! ("CARGO_MANIFEST_DIR")) ; assert ! (manifest_dir . starts_with (&* workspace)) ; }
};
}
