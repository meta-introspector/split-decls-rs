// Generated macro for cargo_pkg_homepage (function)
macro_rules! Depcrate_inputcargo_pkg_homepage {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_homepage"}
// Dependencies: {}
# [doc = " The home page from the manifest of your package."] # [track_caller] pub fn cargo_pkg_homepage () -> Option < String > { to_opt (var_or_panic ("CARGO_PKG_HOMEPAGE")) . map (to_string) }
};
}
