// Generated macro for cargo_pkg_version_major (function)
macro_rules! Depcrate_inputcargo_pkg_version_major {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_version_major"}
// Dependencies: {}
# [doc = " The major version of your package."] # [track_caller] pub fn cargo_pkg_version_major () -> u64 { to_parsed (var_or_panic ("CARGO_PKG_VERSION_MAJOR")) }
};
}
