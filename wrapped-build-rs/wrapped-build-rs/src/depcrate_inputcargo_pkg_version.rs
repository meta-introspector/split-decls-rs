// Generated macro for cargo_pkg_version (function)
macro_rules! Depcrate_inputcargo_pkg_version {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_version"}
// Dependencies: {}
# [doc = " The full version of your package."] # [track_caller] pub fn cargo_pkg_version () -> String { to_string (var_or_panic ("CARGO_PKG_VERSION")) }
};
}
