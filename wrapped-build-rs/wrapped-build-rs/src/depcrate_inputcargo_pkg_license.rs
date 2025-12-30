// Generated macro for cargo_pkg_license (function)
macro_rules! Depcrate_inputcargo_pkg_license {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_license"}
// Dependencies: {}
# [doc = " The license from the manifest of your package."] # [track_caller] pub fn cargo_pkg_license () -> Option < String > { to_opt (var_or_panic ("CARGO_PKG_LICENSE")) . map (to_string) }
};
}
