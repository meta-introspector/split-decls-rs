// Generated macro for shutdown (function)
macro_rules! Depcrate_ssl_testshutdown {
() => {
// Module: crate::ssl::test
// Provides: {"shutdown"}
// Dependencies: {}
# [test] fn shutdown () { let mut server = Server :: builder () ; server . io_cb (| mut s | { assert_eq ! (s . read (& mut [0]) . unwrap () , 0) ; assert_eq ! (s . shutdown () . unwrap () , ShutdownResult :: Received) ; }) ; let server = server . build () ; let mut s = server . client () . connect () ; assert_eq ! (s . get_shutdown () , ShutdownState :: empty ()) ; assert_eq ! (s . shutdown () . unwrap () , ShutdownResult :: Sent) ; assert_eq ! (s . get_shutdown () , ShutdownState :: SENT) ; assert_eq ! (s . shutdown () . unwrap () , ShutdownResult :: Received) ; assert_eq ! (s . get_shutdown () , ShutdownState :: SENT | ShutdownState :: RECEIVED) ; }
};
}
