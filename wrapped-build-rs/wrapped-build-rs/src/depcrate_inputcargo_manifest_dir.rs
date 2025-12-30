// Generated macro for cargo_manifest_dir (function)
macro_rules! Depcrate_inputcargo_manifest_dir {
() => {
// Module: crate::input
// Provides: {"cargo_manifest_dir"}
// Dependencies: {}
# [doc = " The directory containing the manifest for the package being built (the package"] # [doc = " containing the build script)."] # [doc = ""] # [doc = " Also note that this is the value of the current"] # [doc = " working directory of the build script when it starts."] # [track_caller] pub fn cargo_manifest_dir () -> PathBuf { to_path (var_or_panic ("CARGO_MANIFEST_DIR")) }
};
}
