// Generated macro for cargo_manifest_path (function)
macro_rules! Depcrate_inputcargo_manifest_path {
() => {
// Module: crate::input
// Provides: {"cargo_manifest_path"}
// Dependencies: {}
# [doc = " The path to the manifest of your package."] # [track_caller] pub fn cargo_manifest_path () -> PathBuf { ENV . get ("CARGO_MANIFEST_PATH") . map (to_path) . unwrap_or_else (| | { let mut path = cargo_manifest_dir () ; path . push ("Cargo.toml") ; path }) }
};
}
