// Generated macro for voluntary_ack_with_large_datagrams (function)
macro_rules! Depcrate_testsvoluntary_ack_with_large_datagrams {
() => {
// Module: crate::tests
// Provides: {"voluntary_ack_with_large_datagrams"}
// Dependencies: {}
# [doc = " Verify that an ACK prompted by receipt of many non-ACK-eliciting packets is sent alongside"] # [doc = " outgoing application datagrams too large to coexist in the same packet with it."] # [test] fn voluntary_ack_with_large_datagrams () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; let initial_datagrams = pair . client_conn_mut (client_ch) . stats () . udp_tx . datagrams ; const COUNT : usize = 256 ; for _ in 0 .. COUNT { let max_size = pair . client_datagrams (client_ch) . max_size () . unwrap () ; pair . client_datagrams (client_ch) . send (vec ! [0 ; max_size] . into () , true) . unwrap () ; pair . drive () ; } let final_datagrams = pair . client_conn_mut (client_ch) . stats () . udp_tx . datagrams ; assert_ne ! (final_datagrams - initial_datagrams , COUNT as u64 , "client should have sent some ACK-only packets") ; }
};
}
