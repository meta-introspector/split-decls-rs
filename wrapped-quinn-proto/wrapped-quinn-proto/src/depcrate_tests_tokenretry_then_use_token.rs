// Generated macro for retry_then_use_token (function)
macro_rules! Depcrate_tests_tokenretry_then_use_token {
() => {
// Module: crate::tests::token
// Provides: {"retry_then_use_token"}
// Dependencies: {}
# [test] fn retry_then_use_token () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let client_config = client_config () ; pair . server . handle_incoming = Box :: new (validate_incoming) ; let (client_ch , _server_ch) = pair . connect_with (client_config . clone ()) ; pair . client . connections . get_mut (& client_ch) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; pair . server . handle_incoming = Box :: new (| incoming | { assert ! (incoming . remote_address_validated ()) ; assert ! (incoming . may_retry ()) ; IncomingConnectionBehavior :: Accept }) ; let (client_ch_2 , _server_ch_2) = pair . connect_with (client_config) ; pair . client . connections . get_mut (& client_ch_2) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; }
};
}
