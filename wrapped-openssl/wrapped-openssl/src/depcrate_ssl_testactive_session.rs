// Generated macro for active_session (function)
macro_rules! Depcrate_ssl_testactive_session {
() => {
// Module: crate::ssl::test
// Provides: {"active_session"}
// Dependencies: {}
# [doc = " LibreSSL 3.2.1 enabled TLSv1.3 by default for clients and sessions do"] # [doc = " not work due to lack of PSK support. The test passes with NO_TLSV1_3,"] # [doc = " but let's ignore it until LibreSSL supports it out of the box."] # [test] # [cfg_attr (libressl , ignore)] fn active_session () { let server = Server :: builder () . build () ; let s = server . client () . connect () ; let session = s . ssl () . session () . unwrap () ; let len = session . master_key_len () ; let mut buf = vec ! [0 ; len - 1] ; let copied = session . master_key (& mut buf) ; assert_eq ! (copied , buf . len ()) ; let mut buf = vec ! [0 ; len + 1] ; let copied = session . master_key (& mut buf) ; assert_eq ! (copied , len) ; }
};
}
