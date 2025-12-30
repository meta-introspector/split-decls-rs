// Generated macro for congested_tail_loss (function)
macro_rules! Depcrate_testscongested_tail_loss {
() => {
// Module: crate::tests
// Provides: {"congested_tail_loss"}
// Dependencies: {}
# [test] fn congested_tail_loss () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; const TARGET : u64 = 2048 ; assert ! (pair . client_conn_mut (client_ch) . congestion_window () > TARGET) ; let s = pair . client_streams (client_ch) . open (Dir :: Uni) . unwrap () ; while pair . client_conn_mut (client_ch) . congestion_window () > TARGET { let n = pair . client_send (client_ch , s) . write (& [42 ; 1024]) . unwrap () ; assert_eq ! (n , 1024) ; pair . drive_client () ; } assert ! (! pair . server . inbound . is_empty ()) ; pair . server . inbound . clear () ; info ! ("recovering") ; pair . drive () ; assert ! (pair . client_conn_mut (client_ch) . congestion_window () > TARGET) ; pair . client_send (client_ch , s) . write (& [42 ; 1024]) . unwrap () ; }
};
}
