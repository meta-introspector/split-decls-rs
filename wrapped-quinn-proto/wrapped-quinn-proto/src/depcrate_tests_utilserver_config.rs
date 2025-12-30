// Generated macro for server_config (function)
macro_rules! Depcrate_tests_utilserver_config {
() => {
// Module: crate::tests::util
// Provides: {"server_config"}
// Dependencies: {}
pub (super) fn server_config () -> ServerConfig { let mut config = ServerConfig :: with_crypto (Arc :: new (server_crypto ())) ; if ! cfg ! (feature = "bloom") { config . validation_token . sent (2) . log (Arc :: new (SimpleTokenLog :: default ())) ; } config }
};
}
