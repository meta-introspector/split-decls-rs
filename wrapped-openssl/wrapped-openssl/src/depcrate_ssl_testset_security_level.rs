// Generated macro for set_security_level (function)
macro_rules! Depcrate_ssl_testset_security_level {
() => {
// Module: crate::ssl::test
// Provides: {"set_security_level"}
// Dependencies: {}
# [test] # [cfg (ossl110)] fn set_security_level () { let mut ctx = SslContext :: builder (SslMethod :: tls_server ()) . unwrap () ; ctx . set_security_level (3) ; let ctx = ctx . build () ; assert_eq ! (3 , ctx . security_level ()) ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; ssl . set_security_level (4) ; let ssl = ssl ; assert_eq ! (4 , ssl . security_level ()) ; }
};
}
