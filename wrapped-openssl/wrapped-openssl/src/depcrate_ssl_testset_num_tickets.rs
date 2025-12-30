// Generated macro for set_num_tickets (function)
macro_rules! Depcrate_ssl_testset_num_tickets {
() => {
// Module: crate::ssl::test
// Provides: {"set_num_tickets"}
// Dependencies: {}
# [test] # [cfg (ossl111)] fn set_num_tickets () { let mut ctx = SslContext :: builder (SslMethod :: tls_server ()) . unwrap () ; ctx . set_num_tickets (3) . unwrap () ; let ctx = ctx . build () ; assert_eq ! (3 , ctx . num_tickets ()) ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; ssl . set_num_tickets (5) . unwrap () ; let ssl = ssl ; assert_eq ! (5 , ssl . num_tickets ()) ; }
};
}
