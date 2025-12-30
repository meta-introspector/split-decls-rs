// Generated macro for missing_initial_source_connection_id (function)
macro_rules! Depcrate_testsmissing_initial_source_connection_id {
() => {
// Module: crate::tests
// Provides: {"missing_initial_source_connection_id"}
// Dependencies: {}
# [rstest] fn missing_initial_source_connection_id (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut pipe = test_utils :: Pipe :: new (cc_algorithm_name) . unwrap () ; pipe . client . local_transport_params . initial_source_connection_id = None ; assert_eq ! (pipe . client . encode_transport_params () , Ok (())) ; let (len , _) = pipe . client . send (& mut buf) . unwrap () ; assert_eq ! (pipe . server_recv (& mut buf [.. len]) , Err (Error :: InvalidTransportParam)) ; }
};
}
