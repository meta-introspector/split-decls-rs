// Generated macro for finish_retransmit (function)
macro_rules! Depcrate_testsfinish_retransmit {
() => {
// Module: crate::tests
// Provides: {"finish_retransmit"}
// Dependencies: {}
# [test] # [doc = " Ensure that we don't yield a finish event while there's still unacknowledged data"] fn finish_retransmit () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; const MSG : & [u8] = b"hello" ; pair . client_send (client_ch , s) . write (MSG) . unwrap () ; pair . drive_client () ; pair . server . inbound . clear () ; pair . client_send (client_ch , s) . finish () . unwrap () ; pair . drive_client () ; pair . drive_server () ; pair . drive_client () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: Stream (StreamEvent :: Finished { id })) if id == s) ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: Stream (StreamEvent :: Opened { dir : Dir :: Uni }))) ; assert_matches ! (pair . server_streams (server_ch) . accept (Dir :: Uni) , Some (stream) if stream == s) ; let mut recv = pair . server_recv (server_ch , s) ; let mut chunks = recv . read (false) . unwrap () ; assert_matches ! (chunks . next (usize :: MAX) , Ok (Some (chunk)) if chunk . offset == 0 && chunk . bytes == MSG) ; assert_matches ! (chunks . next (usize :: MAX) , Ok (None)) ; let _ = chunks . finalize () ; }
};
}
