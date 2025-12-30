// Generated macro for get_workspace_manifest (function)
macro_rules! Depcrate_dependenciesget_workspace_manifest {
() => {
// Module: crate::dependencies
// Provides: {"get_workspace_manifest"}
// Dependencies: {}
pub (crate) fn get_workspace_manifest (manifest_dir : & Path) -> WorkspaceManifest { try_get_workspace_manifest (manifest_dir) . unwrap_or_default () }
};
}
