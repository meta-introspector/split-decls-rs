// Generated macro for cargo_pkg_rust_version (function)
macro_rules! Depcrate_inputcargo_pkg_rust_version {
() => {
// Module: crate::input
// Provides: {"cargo_pkg_rust_version"}
// Dependencies: {}
# [doc = " The Rust version from the manifest of your package. Note that this is the"] # [doc = " minimum Rust version supported by the package, not the current Rust version."] # [track_caller] pub fn cargo_pkg_rust_version () -> Option < String > { to_opt (var_or_panic ("CARGO_PKG_RUST_VERSION")) . map (to_string) }
};
}
