// Generated macro for cargo_pkg_name (function)
macro_rules! Depcrate_inputcargo_pkg_name {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_name"}
// Dependencies: {}
# [doc = " The name of your package."] # [track_caller] pub fn cargo_pkg_name () -> String { to_string (var_or_panic ("CARGO_PKG_NAME")) }
};
}
