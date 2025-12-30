// Generated macro for impl_896 (impl)
macro_rules! Depcrate_config_tree_sections_credentialimpl_896 {
() => {
// Module: crate::config::tree::sections::credential
// Provides: {"impl_896"}
// Dependencies: {}
impl UrlParameter { # [doc = " The `credential.<url>.helper` key."] pub const HELPER : keys :: Program = keys :: Program :: new_program ("helper" , & Credential :: URL_PARAMETER) ; # [doc = " The `credential.<url>.username` key."] pub const USERNAME : keys :: Any = keys :: Any :: new ("username" , & Credential :: URL_PARAMETER) ; # [doc = " The `credential.<url>.useHttpPath` key."] pub const USE_HTTP_PATH : keys :: Boolean = keys :: Boolean :: new_boolean ("useHttpPath" , & Credential :: URL_PARAMETER) ; }
};
}
