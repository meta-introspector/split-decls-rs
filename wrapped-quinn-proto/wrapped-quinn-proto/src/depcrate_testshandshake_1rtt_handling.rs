// Generated macro for handshake_1rtt_handling (function)
macro_rules! Depcrate_testshandshake_1rtt_handling {
() => {
// Module: crate::tests
// Provides: {"handshake_1rtt_handling"}
// Dependencies: {}
# [test] fn handshake_1rtt_handling () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let client_ch = pair . begin_connect (client_config ()) ; pair . drive_client () ; pair . drive_server () ; let server_ch = pair . server . assert_accept () ; pair . client . drive (pair . time , pair . server . addr) ; pair . client . delay_outbound () ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; const MSG : & [u8] = b"hello" ; pair . client_send (client_ch , s) . write (MSG) . unwrap () ; pair . client_send (client_ch , s) . finish () . unwrap () ; pair . client . drive (pair . time , pair . server . addr) ; pair . client . finish_delay () ; pair . drive () ; assert ! (pair . client_conn_mut (client_ch) . stats () . path . lost_packets != 0) ; let mut recv = pair . server_recv (server_ch , s) ; let mut chunks = recv . read (false) . unwrap () ; assert_matches ! (chunks . next (usize :: MAX) , Ok (Some (chunk)) if chunk . offset == 0 && chunk . bytes == MSG) ; let _ = chunks . finalize () ; }
};
}
