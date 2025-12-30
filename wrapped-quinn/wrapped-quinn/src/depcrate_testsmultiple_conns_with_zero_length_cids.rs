// Generated macro for multiple_conns_with_zero_length_cids (function)
macro_rules! Depcrate_testsmultiple_conns_with_zero_length_cids {
() => {
// Module: crate::tests
// Provides: {"multiple_conns_with_zero_length_cids"}
// Dependencies: {}
# [tokio :: test] async fn multiple_conns_with_zero_length_cids () { let _guard = subscribe () ; let mut factory = EndpointFactory :: new () ; factory . endpoint_config . cid_generator (| | Box :: new (RandomConnectionIdGenerator :: new (0))) ; let server = { let _guard = error_span ! ("server") . entered () ; factory . endpoint () } ; let server_addr = server . local_addr () . unwrap () ; let client1 = { let _guard = error_span ! ("client1") . entered () ; factory . endpoint () } ; let client2 = { let _guard = error_span ! ("client2") . entered () ; factory . endpoint () } ; let client1 = async move { let conn = client1 . connect (server_addr , "localhost") . unwrap () . await . unwrap () ; conn . closed () . await ; } . instrument (error_span ! ("client1")) ; let client2 = async move { let conn = client2 . connect (server_addr , "localhost") . unwrap () . await . unwrap () ; conn . closed () . await ; } . instrument (error_span ! ("client2")) ; let server = async move { let client1 = server . accept () . await . unwrap () . await . unwrap () ; let client2 = server . accept () . await . unwrap () . await . unwrap () ; client1 . close (42u32 . into () , & []) ; client2 . close (42u32 . into () , & []) ; } . instrument (error_span ! ("server")) ; tokio :: join ! (client1 , client2 , server) ; }
};
}
