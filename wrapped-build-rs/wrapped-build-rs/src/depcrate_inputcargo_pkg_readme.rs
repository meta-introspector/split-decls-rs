// Generated macro for cargo_pkg_readme (function)
macro_rules! Depcrate_inputcargo_pkg_readme {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_readme"}
// Dependencies: {}
# [doc = " Path to the README file of your package."] # [track_caller] pub fn cargo_pkg_readme () -> Option < PathBuf > { to_opt (var_or_panic ("CARGO_PKG_README")) . map (to_path) }
};
}
