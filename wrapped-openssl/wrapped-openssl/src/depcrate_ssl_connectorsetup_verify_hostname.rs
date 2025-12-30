// Generated macro for setup_verify_hostname (function)
macro_rules! Depcrate_ssl_connectorsetup_verify_hostname {
() => {
// Module: crate::ssl::connector
// Provides: {"setup_verify_hostname"}
// Dependencies: {}
fn setup_verify_hostname (ssl : & mut SslRef , domain : & str) -> Result < () , ErrorStack > { use crate :: x509 :: verify :: X509CheckFlags ; let param = ssl . param_mut () ; param . set_hostflags (X509CheckFlags :: NO_PARTIAL_WILDCARDS) ; match domain . parse () { Ok (ip) => param . set_ip (ip) , Err (_) => param . set_host (domain) , } }
};
}
