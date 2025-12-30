// Generated macro for get_cargo_workspace (function)
macro_rules! Depcrate_envget_cargo_workspace {
() => {
// Module: crate::env
// Provides: {"get_cargo_workspace"}
// Dependencies: {}
# [doc = " Returns the cargo workspace path for a crate manifest, like"] # [doc = " `/Users/janedoe/projects/insta` when passed"] # [doc = " `/Users/janedoe/projects/insta/insta/Cargo.toml`."] # [doc = ""] # [doc = " If `INSTA_WORKSPACE_ROOT` environment variable is set at runtime, use the value as-is."] # [doc = " If `INSTA_WORKSPACE_ROOT` environment variable is set at compile time, use the value as-is."] # [doc = " If `INSTA_WORKSPACE_ROOT` environment variable is not set, use `cargo metadata` to find the workspace root."] pub fn get_cargo_workspace (workspace : Workspace) -> Arc < PathBuf > { if let Ok (workspace_root) = env :: var ("INSTA_WORKSPACE_ROOT") { return PathBuf :: from (workspace_root) . into () ; } let manifest_dir = match workspace { Workspace :: UseAsIs (workspace_root) => return PathBuf :: from (workspace_root) . into () , Workspace :: DetectWithCargo (manifest_dir) => manifest_dir , } ; WORKSPACES . lock () . unwrap () . entry (manifest_dir . to_string ()) . or_insert_with (| | { get_cargo_workspace_from_metadata (manifest_dir) . unwrap_or_else (| e | { eprintln ! ("cargo metadata failed in {manifest_dir}: {e}") ; eprintln ! ("will use manifest directory as fallback") ; Arc :: new (PathBuf :: from (manifest_dir)) }) }) . clone () }
};
}
