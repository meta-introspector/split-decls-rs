// Generated macro for stream_stopped_2 (function)
macro_rules! Depcrate_testsstream_stopped_2 {
() => {
// Module: crate::tests
// Provides: {"stream_stopped_2"}
// Dependencies: {}
# [tokio :: test] async fn stream_stopped_2 () { let _guard = subscribe () ; let endpoint = endpoint () ; let (conn , _server_conn) = tokio :: try_join ! (endpoint . connect (endpoint . local_addr () . unwrap () , "localhost") . unwrap () , async { endpoint . accept () . await . unwrap () . await }) . unwrap () ; let send_stream = conn . open_uni () . await . unwrap () ; let stopped = tokio :: time :: timeout (Duration :: from_millis (100) , send_stream . stopped ()) . instrument (error_span ! ("stopped")) ; tokio :: pin ! (stopped) ; tokio :: select ! { biased ; _x = & mut stopped => { } , _x = std :: future :: ready (()) => { } } drop (send_stream) ; let res = stopped . await ; assert_eq ! (res , Ok (Ok (None))) ; }
};
}
