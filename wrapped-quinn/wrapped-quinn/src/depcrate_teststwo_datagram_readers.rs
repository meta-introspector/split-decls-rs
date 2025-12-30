// Generated macro for two_datagram_readers (function)
macro_rules! Depcrate_teststwo_datagram_readers {
() => {
// Module: crate::tests
// Provides: {"two_datagram_readers"}
// Dependencies: {}
# [tokio :: test] async fn two_datagram_readers () { let _guard = subscribe () ; let endpoint = endpoint () ; let (client , server) = tokio :: join ! (endpoint . connect (endpoint . local_addr () . unwrap () , "localhost") . unwrap () , async { endpoint . accept () . await . unwrap () . await }) ; let client = client . unwrap () ; let server = server . unwrap () ; let done = tokio :: sync :: Notify :: new () ; let (a , b , ()) = tokio :: join ! (async { let x = client . read_datagram () . await . unwrap () ; done . notify_waiters () ; x } , async { let x = client . read_datagram () . await . unwrap () ; done . notify_waiters () ; x } , async { server . send_datagram (b"one" [..] . into ()) . unwrap () ; done . notified () . await ; server . send_datagram_wait (b"two" [..] . into ()) . await . unwrap () ; }) ; assert ! (* a == * b"one" || * b == * b"one") ; assert ! (* a == * b"two" || * b == * b"two") ; }
};
}
