// Generated macro for impl_894 (impl)
macro_rules! Depcrate_config_tree_sections_credentialimpl_894 {
() => {
// Module: crate::config::tree::sections::credential
// Provides: {"impl_894"}
// Dependencies: {}
impl Credential { # [doc = " The `credential.helper` key."] pub const HELPER : keys :: Program = keys :: Program :: new_program ("helper" , & config :: Tree :: CREDENTIAL) ; # [doc = " The `credential.username` key."] pub const USERNAME : keys :: Any = keys :: Any :: new ("username" , & config :: Tree :: CREDENTIAL) ; # [doc = " The `credential.useHttpPath` key."] pub const USE_HTTP_PATH : keys :: Boolean = keys :: Boolean :: new_boolean ("useHttpPath" , & config :: Tree :: CREDENTIAL) ; # [doc = " The `credential.<url>` subsection"] pub const URL_PARAMETER : UrlParameter = UrlParameter ; }
};
}
