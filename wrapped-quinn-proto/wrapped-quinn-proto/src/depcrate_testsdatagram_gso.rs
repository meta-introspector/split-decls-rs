// Generated macro for datagram_gso (function)
macro_rules! Depcrate_testsdatagram_gso {
() => {
// Module: crate::tests
// Provides: {"datagram_gso"}
// Dependencies: {}
# [test] fn datagram_gso () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; let initial_ios = pair . client_conn_mut (client_ch) . stats () . udp_tx . ios ; let initial_bytes = pair . client_conn_mut (client_ch) . stats () . udp_tx . bytes ; info ! ("sending") ; const DATAGRAM_LEN : usize = 1024 ; const DATAGRAMS : usize = 10 ; for _ in 0 .. DATAGRAMS { pair . client_datagrams (client_ch) . send (Bytes :: from_static (& [0 ; DATAGRAM_LEN]) , false) . unwrap () ; } pair . drive () ; let final_ios = pair . client_conn_mut (client_ch) . stats () . udp_tx . ios ; let final_bytes = pair . client_conn_mut (client_ch) . stats () . udp_tx . bytes ; assert_eq ! (final_ios - initial_ios , 1) ; assert_eq ! (final_bytes - initial_bytes , ((29 + DATAGRAM_LEN) * DATAGRAMS) as u64) ; }
};
}
