// Generated macro for to_toml_string (function)
macro_rules! Depcrate_utilsto_toml_string {
() => {
// Module: crate::utils
// Provides: {"to_toml_string"}
// Dependencies: {}
pub fn to_toml_string (str : String) -> String { toml_edit :: value (str) . to_string () }
};
}
