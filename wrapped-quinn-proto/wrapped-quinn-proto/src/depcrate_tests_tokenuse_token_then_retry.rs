// Generated macro for use_token_then_retry (function)
macro_rules! Depcrate_tests_tokenuse_token_then_retry {
() => {
// Module: crate::tests::token
// Provides: {"use_token_then_retry"}
// Dependencies: {}
# [test] fn use_token_then_retry () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let client_config = client_config () ; let (client_ch , _server_ch) = pair . connect_with (client_config . clone ()) ; pair . client . connections . get_mut (& client_ch) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; pair . server . handle_incoming = Box :: new ({ let mut i = 0 ; move | incoming | { if i == 0 { assert ! (incoming . remote_address_validated ()) ; assert ! (incoming . may_retry ()) ; i += 1 ; IncomingConnectionBehavior :: Retry } else if i == 1 { assert ! (incoming . remote_address_validated ()) ; assert ! (! incoming . may_retry ()) ; i += 1 ; IncomingConnectionBehavior :: Accept } else { panic ! ("too many handle_incoming iterations") } } }) ; let (client_ch_2 , _server_ch_2) = pair . connect_with (client_config) ; pair . client . connections . get_mut (& client_ch_2) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; }
};
}
