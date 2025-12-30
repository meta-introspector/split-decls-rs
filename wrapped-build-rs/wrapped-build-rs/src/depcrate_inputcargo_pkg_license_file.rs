// Generated macro for cargo_pkg_license_file (function)
macro_rules! Depcrate_inputcargo_pkg_license_file {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_license_file"}
// Dependencies: {}
# [doc = " The license file from the manifest of your package."] # [track_caller] pub fn cargo_pkg_license_file () -> Option < PathBuf > { to_opt (var_or_panic ("CARGO_PKG_LICENSE_FILE")) . map (to_path) }
};
}
