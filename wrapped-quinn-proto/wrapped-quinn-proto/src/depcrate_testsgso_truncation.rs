// Generated macro for gso_truncation (function)
macro_rules! Depcrate_testsgso_truncation {
() => {
// Module: crate::tests
// Provides: {"gso_truncation"}
// Dependencies: {}
# [test] fn gso_truncation () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let initial_ios = pair . client_conn_mut (client_ch) . stats () . udp_tx . ios ; info ! ("sending") ; const SIZES : [usize ; 3] = [1024 , 768 , 768] ; for len in SIZES { pair . client_datagrams (client_ch) . send (vec ! [0 ; len] . into () , false) . unwrap () ; } pair . drive () ; let final_ios = pair . client_conn_mut (client_ch) . stats () . udp_tx . ios ; assert_eq ! (final_ios - initial_ios , 2) ; for len in SIZES { assert_eq ! (pair . server_datagrams (server_ch) . recv () . expect ("datagram lost") . len () , len) ; } }
};
}
