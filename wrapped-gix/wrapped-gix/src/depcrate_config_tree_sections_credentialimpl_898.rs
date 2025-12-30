// Generated macro for impl_898 (impl)
macro_rules! Depcrate_config_tree_sections_credentialimpl_898 {
() => {
// Module: crate::config::tree::sections::credential
// Provides: {"impl_898"}
// Dependencies: {}
impl Section for Credential { fn name (& self) -> & str { "credential" } fn keys (& self) -> & [& dyn Key] { & [& Self :: HELPER , & Self :: USERNAME , & Self :: USE_HTTP_PATH] } fn sub_sections (& self) -> & [& dyn Section] { & [& Self :: URL_PARAMETER] } }
};
}
