// Generated macro for instant_close_1 (function)
macro_rules! Depcrate_testsinstant_close_1 {
() => {
// Module: crate::tests
// Provides: {"instant_close_1"}
// Dependencies: {}
# [test] fn instant_close_1 () { let _guard = subscribe () ; let mut pair = Pair :: default () ; info ! ("connecting") ; let client_ch = pair . begin_connect (client_config ()) ; pair . client . connections . get_mut (& client_ch) . unwrap () . close (pair . time , VarInt (0) , Bytes :: new ()) ; pair . drive () ; let server_ch = pair . server . assert_accept () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (ConnectionClose { error_code : TransportErrorCode :: APPLICATION_ERROR , .. }) , })) ; }
};
}
