// Generated macro for packet_splitting_with_default_mtu (function)
macro_rules! Depcrate_testspacket_splitting_with_default_mtu {
() => {
// Module: crate::tests
// Provides: {"packet_splitting_with_default_mtu"}
// Dependencies: {}
# [test] fn packet_splitting_with_default_mtu () { let _guard = subscribe () ; let payload = vec ! [42 ; 1300] ; let mut pair = Pair :: default () ; pair . mtu = 1200 ; let (client_ch , _) = pair . connect () ; pair . drive () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; pair . client_send (client_ch , s) . write (& payload) . unwrap () ; pair . client . drive (pair . time , pair . server . addr) ; assert_eq ! (pair . client . outbound . len () , 2) ; pair . drive_client () ; assert_eq ! (pair . server . inbound . len () , 2) ; }
};
}
