// Generated macro for server_can_send_3_inital_packets (function)
macro_rules! Depcrate_testsserver_can_send_3_inital_packets {
() => {
// Module: crate::tests
// Provides: {"server_can_send_3_inital_packets"}
// Dependencies: {}
# [doc = " Ensures that the server can respond with 3 initial packets during the handshake"] # [doc = " before the anti-amplification limit kicks in when MTUs are similar."] # [test] fn server_can_send_3_inital_packets () { let _guard = subscribe () ; let mut transport = TransportConfig :: default () ; transport . initial_rtt (Duration :: from_millis (10)) ; let transport = Arc :: new (transport) ; let (cert , key) = big_cert_and_key () ; let mut server = server_config_with_cert (cert . clone () , key) ; server . transport_config (transport) ; let client = client_config_with_certs (vec ! [cert]) ; let mut pair = Pair :: new (Default :: default () , server) ; let client_ch = pair . begin_connect (client) ; pair . drive_client () ; pair . drive_server () ; assert_eq ! (pair . client . inbound . len () , 3) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: Connected)) ; }
};
}
