// Generated macro for stop_stream (function)
macro_rules! Depcrate_testsstop_stream {
() => {
// Module: crate::tests
// Provides: {"stop_stream"}
// Dependencies: {}
# [test] fn stop_stream () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; const MSG : & [u8] = b"hello" ; pair . client_send (client_ch , s) . write (MSG) . unwrap () ; pair . drive () ; info ! ("stopping stream") ; const ERROR : VarInt = VarInt (42) ; pair . server_recv (server_ch , s) . stop (ERROR) . unwrap () ; pair . drive () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: Stream (StreamEvent :: Opened { dir : Dir :: Uni }))) ; assert_matches ! (pair . server_streams (server_ch) . accept (Dir :: Uni) , Some (stream) if stream == s) ; assert_matches ! (pair . client_send (client_ch , s) . write (b"foo") , Err (WriteError :: Stopped (ERROR))) ; assert_matches ! (pair . client_send (client_ch , s) . finish () , Err (FinishError :: Stopped (ERROR))) ; }
};
}
