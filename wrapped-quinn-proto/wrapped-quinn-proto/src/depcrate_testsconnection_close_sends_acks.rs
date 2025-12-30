// Generated macro for connection_close_sends_acks (function)
macro_rules! Depcrate_testsconnection_close_sends_acks {
() => {
// Module: crate::tests
// Provides: {"connection_close_sends_acks"}
// Dependencies: {}
# [test] fn connection_close_sends_acks () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _server_ch) = pair . connect () ; let client_acks = pair . client_conn_mut (client_ch) . stats () . frame_rx . acks ; pair . client_conn_mut (client_ch) . ping () ; pair . drive_client () ; let time = pair . time ; pair . server_conn_mut (client_ch) . close (time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; let client_acks_2 = pair . client_conn_mut (client_ch) . stats () . frame_rx . acks ; assert ! (client_acks_2 > client_acks , "Connection close should send pending ACKs") ; }
};
}
