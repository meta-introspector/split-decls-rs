// Generated macro for package_name (function)
macro_rules! Depcrate_cargopackage_name {
() => {
// Module: crate::cargo
// Provides: {"package_name"}
// Dependencies: {}
pub (crate) fn package_name () -> String { env :: var ("CARGO_PKG_NAME") . unwrap_or_else (| _ | "<unknown>" . to_string ()) }
};
}
