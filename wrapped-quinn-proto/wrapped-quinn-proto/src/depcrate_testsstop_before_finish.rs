// Generated macro for stop_before_finish (function)
macro_rules! Depcrate_testsstop_before_finish {
() => {
// Module: crate::tests
// Provides: {"stop_before_finish"}
// Dependencies: {}
# [test] fn stop_before_finish () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; const MSG : & [u8] = b"hello" ; pair . client_send (client_ch , s) . write (MSG) . unwrap () ; pair . drive () ; info ! ("stopping stream") ; const ERROR : VarInt = VarInt (42) ; pair . server_recv (server_ch , s) . stop (ERROR) . unwrap () ; pair . drive () ; assert_matches ! (pair . client_send (client_ch , s) . finish () , Err (FinishError :: Stopped (ERROR))) ; }
};
}
