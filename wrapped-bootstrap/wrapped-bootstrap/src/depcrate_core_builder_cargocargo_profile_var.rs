// Generated macro for cargo_profile_var (function)
macro_rules! Depcrate_core_builder_cargocargo_profile_var {
() => {
// Module: crate::core::builder::cargo
// Provides: {"cargo_profile_var"}
// Dependencies: {}
pub fn cargo_profile_var (name : & str , config : & Config) -> String { let profile = if config . rust_optimize . is_release () { "RELEASE" } else { "DEV" } ; format ! ("CARGO_PROFILE_{profile}_{name}") }
};
}
