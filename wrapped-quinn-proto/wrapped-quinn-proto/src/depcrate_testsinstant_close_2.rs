// Generated macro for instant_close_2 (function)
macro_rules! Depcrate_testsinstant_close_2 {
() => {
// Module: crate::tests
// Provides: {"instant_close_2"}
// Dependencies: {}
# [test] fn instant_close_2 () { let _guard = subscribe () ; let mut pair = Pair :: default () ; info ! ("connecting") ; let client_ch = pair . begin_connect (client_config ()) ; pair . drive_client () ; pair . client . connections . get_mut (& client_ch) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; let server_ch = pair . server . assert_accept () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: HandshakeDataReady)) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (ConnectionClose { error_code : TransportErrorCode :: APPLICATION_ERROR , .. }) , })) ; }
};
}
