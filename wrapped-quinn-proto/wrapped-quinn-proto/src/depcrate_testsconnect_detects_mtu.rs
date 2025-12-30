// Generated macro for connect_detects_mtu (function)
macro_rules! Depcrate_testsconnect_detects_mtu {
() => {
// Module: crate::tests
// Provides: {"connect_detects_mtu"}
// Dependencies: {}
# [test] fn connect_detects_mtu () { let _guard = subscribe () ; let max_udp_payload_and_expected_mtu = & [(1200 , 1200) , (1400 , 1389) , (1500 , 1452)] ; for & (pair_max_udp , expected_mtu) in max_udp_payload_and_expected_mtu { let mut pair = Pair :: default () ; pair . mtu = pair_max_udp ; let (client_ch , server_ch) = pair . connect () ; pair . drive () ; assert_eq ! (pair . client_conn_mut (client_ch) . path_mtu () , expected_mtu) ; assert_eq ! (pair . server_conn_mut (server_ch) . path_mtu () , expected_mtu) ; } }
};
}
