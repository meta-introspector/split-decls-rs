// Generated macro for cargo_pkg_description (function)
macro_rules! Depcrate_inputcargo_pkg_description {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_description"}
// Dependencies: {}
# [doc = " The description from the manifest of your package."] # [track_caller] pub fn cargo_pkg_description () -> Option < String > { to_opt (var_or_panic ("CARGO_PKG_DESCRIPTION")) . map (to_string) }
};
}
