// Generated macro for cargo_pkg_authors (function)
macro_rules! Depcrate_inputcargo_pkg_authors {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_authors"}
// Dependencies: {}
# [doc = " The authors from the manifest of your package."] # [track_caller] pub fn cargo_pkg_authors () -> Vec < String > { to_strings (var_or_panic ("CARGO_PKG_AUTHORS") , ':') }
};
}
