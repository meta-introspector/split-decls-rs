// Generated macro for build_root (function)
macro_rules! Depcrate_path_helpersbuild_root {
() => {
// Module: crate::path_helpers
// Provides: {"build_root"}
// Dependencies: {}
# [doc = " Path to the build directory root."] # [must_use] pub fn build_root () -> PathBuf { env_var ("BUILD_ROOT") . into () }
};
}
