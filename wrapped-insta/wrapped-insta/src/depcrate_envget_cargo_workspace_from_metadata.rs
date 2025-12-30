// Generated macro for get_cargo_workspace_from_metadata (function)
macro_rules! Depcrate_envget_cargo_workspace_from_metadata {
() => {
// Module: crate::env
// Provides: {"get_cargo_workspace_from_metadata"}
// Dependencies: {}
fn get_cargo_workspace_from_metadata (manifest_dir : & str ,) -> Result < Arc < PathBuf > , Box < dyn std :: error :: Error > > { let output = std :: process :: Command :: new (env :: var ("CARGO") . unwrap_or_else (| _ | "cargo" . to_string ())) . args (["metadata" , "--format-version=1" , "--no-deps"]) . current_dir (manifest_dir) . output () ? ; if ! output . status . success () { let stderr = String :: from_utf8_lossy (& output . stderr) ; return Err (format ! ("command failed with {}: {stderr}" , output . status) . into ()) ; } let stdout = std :: str :: from_utf8 (& output . stdout) . map_err (| e | format ! ("invalid UTF-8 in output: {e}")) ? ; let docs = crate :: content :: yaml :: vendored :: yaml :: YamlLoader :: load_from_str (stdout) . map_err (| e | format ! ("failed to parse YAML: {e}")) ? ; let metadata = docs . into_iter () . next () . ok_or ("no content found in YAML") ? ; let workspace_root = metadata ["workspace_root"] . clone () . into_string () . ok_or ("couldn't find 'workspace_root' in metadata") ? ; Ok (Arc :: new (workspace_root . into ())) }
};
}
