// Generated macro for read_after_close (function)
macro_rules! Depcrate_testsread_after_close {
() => {
// Module: crate::tests
// Provides: {"read_after_close"}
// Dependencies: {}
# [test] fn read_after_close () { let _guard = subscribe () ; let runtime = rt_basic () ; let endpoint = { let _guard = runtime . enter () ; endpoint () } ; const MSG : & [u8] = b"goodbye!" ; let endpoint2 = endpoint . clone () ; runtime . spawn (async move { let new_conn = endpoint2 . accept () . await . expect ("endpoint") . await . expect ("connection") ; let mut s = new_conn . open_uni () . await . unwrap () ; s . write_all (MSG) . await . unwrap () ; s . finish () . unwrap () ; _ = s . stopped () . await ; }) ; runtime . block_on (async move { let new_conn = endpoint . connect (endpoint . local_addr () . unwrap () , "localhost") . unwrap () . await . expect ("connect") ; tokio :: time :: sleep (Duration :: from_millis (100)) . await ; let mut stream = new_conn . accept_uni () . await . expect ("incoming streams") ; let msg = stream . read_to_end (usize :: MAX) . await . expect ("read_to_end") ; assert_eq ! (msg , MSG) ; }) ; }
};
}
