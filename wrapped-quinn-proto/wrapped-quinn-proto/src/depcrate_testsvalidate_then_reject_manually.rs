// Generated macro for validate_then_reject_manually (function)
macro_rules! Depcrate_testsvalidate_then_reject_manually {
() => {
// Module: crate::tests
// Provides: {"validate_then_reject_manually"}
// Dependencies: {}
# [test] fn validate_then_reject_manually () { let _guard = subscribe () ; let mut pair = Pair :: default () ; pair . server . handle_incoming = Box :: new ({ let mut i = 0 ; move | incoming | { if incoming . remote_address_validated () { assert_eq ! (i , 1) ; i += 1 ; IncomingConnectionBehavior :: Reject } else { assert_eq ! (i , 0) ; i += 1 ; IncomingConnectionBehavior :: Retry } } }) ; let client_ch = pair . begin_connect (client_config ()) ; pair . drive () ; pair . server . assert_no_accept () ; let client = pair . client . connections . get_mut (& client_ch) . unwrap () ; assert ! (client . is_closed ()) ; assert ! (matches ! (client . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (close) }) if close . error_code == TransportErrorCode :: CONNECTION_REFUSED)) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , None) ; assert_eq ! (pair . client . known_connections () , 0) ; assert_eq ! (pair . client . known_cids () , 0) ; assert_eq ! (pair . server . known_connections () , 0) ; assert_eq ! (pair . server . known_cids () , 0) ; }
};
}
