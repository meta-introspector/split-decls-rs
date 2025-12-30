// Generated macro for pure_sender_voluntarily_acks (function)
macro_rules! Depcrate_testspure_sender_voluntarily_acks {
() => {
// Module: crate::tests
// Provides: {"pure_sender_voluntarily_acks"}
// Dependencies: {}
# [doc = " Verify that an endpoint which receives but does not send ACK-eliciting data still receives ACKs"] # [doc = " occasionally. This is not required for conformance, but makes loss detection more responsive and"] # [doc = " reduces receiver memory use."] # [test] fn pure_sender_voluntarily_acks () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let receiver_acks_initial = pair . server_conn_mut (server_ch) . stats () . frame_rx . acks ; for _ in 0 .. 100 { const MSG : & [u8] = b"hello" ; pair . client_datagrams (client_ch) . send (Bytes :: from_static (MSG) , true) . unwrap () ; pair . drive () ; assert_eq ! (pair . server_datagrams (server_ch) . recv () . unwrap () , MSG) ; } let receiver_acks_final = pair . server_conn_mut (server_ch) . stats () . frame_rx . acks ; assert ! (receiver_acks_final > receiver_acks_initial) ; }
};
}
