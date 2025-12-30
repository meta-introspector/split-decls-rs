// Generated macro for migrate_detects_new_mtu_and_respects_original_peer_max_udp_payload_size (function)
macro_rules! Depcrate_testsmigrate_detects_new_mtu_and_respects_original_peer_max_udp_payload_size {
() => {
// Module: crate::tests
// Provides: {"migrate_detects_new_mtu_and_respects_original_peer_max_udp_payload_size"}
// Dependencies: {}
# [test] fn migrate_detects_new_mtu_and_respects_original_peer_max_udp_payload_size () { let _guard = subscribe () ; let client_max_udp_payload_size : u16 = 1400 ; let server_endpoint_config = EndpointConfig :: default () ; let server = Endpoint :: new (Arc :: new (server_endpoint_config) , Some (Arc :: new (server_config ())) , true , None ,) ; let client_endpoint_config = EndpointConfig { max_udp_payload_size : VarInt :: from (client_max_udp_payload_size) , .. EndpointConfig :: default () } ; let client = Endpoint :: new (Arc :: new (client_endpoint_config) , None , true , None) ; let mut pair = Pair :: new_from_endpoint (client , server) ; pair . mtu = 1300 ; let (client_ch , server_ch) = pair . connect () ; pair . drive () ; assert_eq ! (pair . client_conn_mut (client_ch) . path_mtu () , 1293) ; assert_eq ! (pair . server_conn_mut (server_ch) . path_mtu () , 1300) ; pair . mtu = 1500 ; pair . client . addr = SocketAddr :: new (Ipv4Addr :: new (127 , 0 , 0 , 1) . into () , CLIENT_PORTS . lock () . unwrap () . next () . unwrap () ,) ; pair . client_conn_mut (client_ch) . ping () ; pair . drive () ; assert_eq ! (pair . server_conn_mut (server_ch) . remote_address () , pair . client . addr) ; assert_eq ! (pair . server_conn_mut (server_ch) . path_mtu () , client_max_udp_payload_size) ; assert_eq ! (pair . client_conn_mut (client_ch) . path_mtu () , 1293) ; }
};
}
