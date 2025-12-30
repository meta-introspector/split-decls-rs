// Generated macro for connector_client_server_mozilla_modern_v5 (function)
macro_rules! Depcrate_ssl_testconnector_client_server_mozilla_modern_v5 {
() => {
// Module: crate::ssl::test
// Provides: {"connector_client_server_mozilla_modern_v5"}
// Dependencies: {}
# [test] # [cfg (any (ossl111 , libressl))] fn connector_client_server_mozilla_modern_v5 () { test_mozilla_server (SslAcceptor :: mozilla_modern_v5) ; }
};
}
