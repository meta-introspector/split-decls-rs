// Generated macro for impl_941 (impl)
macro_rules! Depcrate_config_tree_sections_httpimpl_941 {
() => {
// Module: crate::config::tree::sections::http
// Provides: {"impl_941"}
// Dependencies: {}
impl Section for Http { fn name (& self) -> & str { "http" } fn keys (& self) -> & [& dyn Key] { & [& Self :: SSL_VERSION , & Self :: SSL_VERIFY , & Self :: PROXY , & Self :: PROXY_AUTH_METHOD , & Self :: VERSION , & Self :: USER_AGENT , & Self :: EXTRA_HEADER , & Self :: FOLLOW_REDIRECTS , & Self :: LOW_SPEED_TIME , & Self :: LOW_SPEED_LIMIT , & Self :: SCHANNEL_USE_SSL_CA_INFO , & Self :: SSL_CA_INFO , & Self :: SCHANNEL_CHECK_REVOKE ,] } }
};
}
