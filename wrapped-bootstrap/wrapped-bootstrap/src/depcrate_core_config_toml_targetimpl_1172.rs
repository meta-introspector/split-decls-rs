// Generated macro for impl_1172 (impl)
macro_rules! Depcrate_core_config_toml_targetimpl_1172 {
() => {
// Module: crate::core::config::toml::target
// Provides: {"impl_1172"}
// Dependencies: {}
impl Target { pub fn from_triple (triple : & str) -> Self { let mut target : Self = Default :: default () ; if ! build_helper :: targets :: target_supports_std (triple) { target . no_std = true ; } if triple . contains ("emscripten") { target . runner = Some ("node" . into ()) ; } target } }
};
}
