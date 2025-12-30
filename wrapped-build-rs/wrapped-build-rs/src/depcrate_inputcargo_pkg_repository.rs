// Generated macro for cargo_pkg_repository (function)
macro_rules! Depcrate_inputcargo_pkg_repository {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_repository"}
// Dependencies: {}
# [doc = " The repository from the manifest of your package."] # [track_caller] pub fn cargo_pkg_repository () -> Option < String > { to_opt (var_or_panic ("CARGO_PKG_REPOSITORY")) . map (to_string) }
};
}
