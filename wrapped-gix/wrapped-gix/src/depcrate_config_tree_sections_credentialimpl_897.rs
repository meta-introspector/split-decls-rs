// Generated macro for impl_897 (impl)
macro_rules! Depcrate_config_tree_sections_credentialimpl_897 {
() => {
// Module: crate::config::tree::sections::credential
// Provides: {"impl_897"}
// Dependencies: {}
impl Section for UrlParameter { fn name (& self) -> & str { "<url>" } fn keys (& self) -> & [& dyn Key] { & [& Self :: HELPER , & Self :: USERNAME , & Self :: USE_HTTP_PATH] } fn parent (& self) -> Option < & dyn Section > { Some (& config :: Tree :: CREDENTIAL) } }
};
}
