// Generated macro for handshake_anti_deadlock_probe (function)
macro_rules! Depcrate_testshandshake_anti_deadlock_probe {
() => {
// Module: crate::tests
// Provides: {"handshake_anti_deadlock_probe"}
// Dependencies: {}
# [doc = " Ensures that the client sends an anti-deadlock probe after an incomplete server's first flight"] # [test] fn handshake_anti_deadlock_probe () { let _guard = subscribe () ; let (cert , key) = big_cert_and_key () ; let server = server_config_with_cert (cert . clone () , key) ; let client = client_config_with_certs (vec ! [cert]) ; let mut pair = Pair :: new (Default :: default () , server) ; let client_ch = pair . begin_connect (client) ; pair . drive_client () ; pair . drive_server () ; pair . drive_client () ; pair . server . inbound . clear () ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: Connected)) ; }
};
}
