// Generated macro for cargo_pkg_version_patch (function)
macro_rules! Depcrate_inputcargo_pkg_version_patch {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_version_patch"}
// Dependencies: {}
# [doc = " The patch version of your package."] # [track_caller] pub fn cargo_pkg_version_patch () -> u64 { to_parsed (var_or_panic ("CARGO_PKG_VERSION_PATCH")) }
};
}
