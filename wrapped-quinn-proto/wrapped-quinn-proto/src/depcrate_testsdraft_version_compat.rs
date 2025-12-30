// Generated macro for draft_version_compat (function)
macro_rules! Depcrate_testsdraft_version_compat {
() => {
// Module: crate::tests
// Provides: {"draft_version_compat"}
// Dependencies: {}
# [test] fn draft_version_compat () { let _guard = subscribe () ; let mut client_config = client_config () ; client_config . version (0xff00_0020) ; let mut pair = Pair :: default () ; let (client_ch , server_ch) = pair . connect_with (client_config) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; assert ! (pair . client_conn_mut (client_ch) . using_ecn ()) ; assert ! (pair . server_conn_mut (server_ch) . using_ecn ()) ; const REASON : & [u8] = b"whee" ; info ! ("closing") ; pair . client . connections . get_mut (& client_ch) . unwrap () . close (pair . time , VarInt (42) , REASON . into () ,) ; pair . drive () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ApplicationClosed (ApplicationClose { error_code : VarInt (42) , ref reason }) }) if reason == REASON) ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; }
};
}
