// Generated macro for instant_server_close (function)
macro_rules! Depcrate_testsinstant_server_close {
() => {
// Module: crate::tests
// Provides: {"instant_server_close"}
// Dependencies: {}
# [test] fn instant_server_close () { let _guard = subscribe () ; let mut pair = Pair :: default () ; info ! ("connecting") ; pair . begin_connect (client_config ()) ; pair . drive_client () ; pair . server . drive_incoming (pair . time , pair . client . addr) ; let server_ch = pair . server . assert_accept () ; info ! ("closing") ; pair . server . connections . get_mut (& server_ch) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (server_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (ConnectionClose { error_code : TransportErrorCode :: APPLICATION_ERROR , .. }) , })) ; }
};
}
