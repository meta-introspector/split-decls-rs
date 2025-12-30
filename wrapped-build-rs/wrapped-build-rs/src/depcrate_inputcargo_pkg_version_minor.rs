// Generated macro for cargo_pkg_version_minor (function)
macro_rules! Depcrate_inputcargo_pkg_version_minor {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_version_minor"}
// Dependencies: {}
# [doc = " The minor version of your package."] # [track_caller] pub fn cargo_pkg_version_minor () -> u64 { to_parsed (var_or_panic ("CARGO_PKG_VERSION_MINOR")) }
};
}
