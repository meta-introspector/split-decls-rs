// Generated macro for domain_as_uri (function)
macro_rules! Depcrate_client_legacy_clientdomain_as_uri {
() => {
// Module: crate::client::legacy::client
// Provides: {"domain_as_uri"}
// Dependencies: {}
fn domain_as_uri ((scheme , auth) : PoolKey) -> Uri { http :: uri :: Builder :: new () . scheme (scheme) . authority (auth) . path_and_query ("/") . build () . expect ("domain is valid Uri") }
};
}
