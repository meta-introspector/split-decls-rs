// Generated macro for impl_225 (impl)
macro_rules! Depcrate_providersimpl_225 {
() => {
// Module: crate::providers
// Provides: {"impl_225"}
// Dependencies: {}
impl ProviderConfig { pub fn is_enabled (& self) -> bool { self . enabled . unwrap_or (true) } # [doc = " Get the API key or OAuth provider ID"] pub fn get_auth_credential (& self) -> Option < String > { match self . auth_type { AuthType :: ApiKey => self . api_key . clone () , AuthType :: OAuth => self . oauth_provider . clone () , } } }
};
}
