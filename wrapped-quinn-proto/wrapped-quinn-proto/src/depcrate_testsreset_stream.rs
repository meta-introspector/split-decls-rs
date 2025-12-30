// Generated macro for reset_stream (function)
macro_rules! Depcrate_testsreset_stream {
() => {
// Module: crate::tests
// Provides: {"reset_stream"}
// Dependencies: {}
# [test] fn reset_stream () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; const MSG : & [u8] = b"hello" ; pair . client_send (client_ch , s) . write (MSG) . unwrap () ; pair . drive () ; info ! ("resetting stream") ; const ERROR : VarInt = VarInt (42) ; pair . client_send (client_ch , s) . reset (ERROR) . unwrap () ; pair . drive () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: Stream (StreamEvent :: Opened { dir : Dir :: Uni }))) ; assert_matches ! (pair . server_streams (server_ch) . accept (Dir :: Uni) , Some (stream) if stream == s) ; let mut recv = pair . server_recv (server_ch , s) ; let mut chunks = recv . read (false) . unwrap () ; assert_matches ! (chunks . next (usize :: MAX) , Err (ReadError :: Reset (ERROR))) ; let _ = chunks . finalize () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; }
};
}
