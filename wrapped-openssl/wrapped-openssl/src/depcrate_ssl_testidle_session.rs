// Generated macro for idle_session (function)
macro_rules! Depcrate_ssl_testidle_session {
() => {
// Module: crate::ssl::test
// Provides: {"idle_session"}
// Dependencies: {}
# [test] fn idle_session () { let ctx = SslContext :: builder (SslMethod :: tls ()) . unwrap () . build () ; let ssl = Ssl :: new (& ctx) . unwrap () ; assert ! (ssl . session () . is_none ()) ; }
};
}
