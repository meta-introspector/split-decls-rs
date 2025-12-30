// Generated macro for read_cargo_toml (function)
macro_rules! Depcrateread_cargo_toml {
() => {
// Module: crate
// Provides: {"read_cargo_toml"}
// Dependencies: {}
fn read_cargo_toml (manifest_path : & Path , workspace_manifest_path : & Path , manifest_ts : SystemTime , workspace_manifest_ts : SystemTime ,) -> Result < CacheEntry , Error > { let manifest = open_cargo_toml (manifest_path) ? ; let workspace_dependencies = if manifest_path != workspace_manifest_path { let workspace_manifest = open_cargo_toml (workspace_manifest_path) ? ; extract_workspace_dependencies (& workspace_manifest) ? } else { extract_workspace_dependencies (& manifest) ? } ; let crate_names = extract_crate_names (& manifest , workspace_dependencies) ? ; Ok (CacheEntry { manifest_ts , workspace_manifest_ts , crate_names , workspace_manifest_path : workspace_manifest_path . to_path_buf () , }) }
};
}
