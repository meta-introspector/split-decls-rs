// Generated macro for reject_manually (function)
macro_rules! Depcrate_testsreject_manually {
() => {
// Module: crate::tests
// Provides: {"reject_manually"}
// Dependencies: {}
# [test] fn reject_manually () { let _guard = subscribe () ; let mut pair = Pair :: default () ; pair . server . handle_incoming = Box :: new (| _ | IncomingConnectionBehavior :: Reject) ; let client_ch = pair . begin_connect (client_config ()) ; pair . drive () ; pair . server . assert_no_accept () ; let client = pair . client . connections . get_mut (& client_ch) . unwrap () ; assert ! (client . is_closed ()) ; assert ! (matches ! (client . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: ConnectionClosed (close) }) if close . error_code == TransportErrorCode :: CONNECTION_REFUSED)) ; }
};
}
