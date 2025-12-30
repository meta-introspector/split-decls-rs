// Generated macro for cargo_pkg_version_pre (function)
macro_rules! Depcrate_inputcargo_pkg_version_pre {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_version_pre"}
// Dependencies: {}
# [doc = " The pre-release version of your package."] # [track_caller] pub fn cargo_pkg_version_pre () -> Option < String > { to_opt (var_or_panic ("CARGO_PKG_VERSION_PRE")) . map (to_string) }
};
}
