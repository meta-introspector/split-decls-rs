// Generated macro for crate_name (function)
macro_rules! Depcrate_cargocrate_name {
() => {
// Module: crate::cargo
// Provides: {"crate_name"}
// Dependencies: {}
pub (crate) fn crate_name () -> String { env :: var ("CARGO_CRATE_NAME") . unwrap_or_else (| _ | "<unknown>" . to_string ()) }
};
}
