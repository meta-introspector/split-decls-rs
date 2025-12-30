// Generated macro for change_idle_timeout (function)
macro_rules! Depcrate_testschange_idle_timeout {
() => {
// Module: crate::tests
// Provides: {"change_idle_timeout"}
// Dependencies: {}
# [rstest] fn change_idle_timeout (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (0x1) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . set_max_idle_timeout (999999) ; config . verify_peer (false) ; let mut pipe = test_utils :: Pipe :: with_client_config (& mut config) . unwrap () ; assert_eq ! (pipe . client . local_transport_params . max_idle_timeout , 999999) ; assert_eq ! (pipe . client . peer_transport_params . max_idle_timeout , 0) ; assert_eq ! (pipe . server . local_transport_params . max_idle_timeout , 0) ; assert_eq ! (pipe . server . peer_transport_params . max_idle_timeout , 0) ; pipe . client . set_max_idle_timeout (456000) . unwrap () ; pipe . server . set_max_idle_timeout (234000) . unwrap () ; assert_eq ! (pipe . client . local_transport_params . max_idle_timeout , 456000) ; assert_eq ! (pipe . client . peer_transport_params . max_idle_timeout , 0) ; assert_eq ! (pipe . server . local_transport_params . max_idle_timeout , 234000) ; assert_eq ! (pipe . server . peer_transport_params . max_idle_timeout , 0) ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . idle_timeout () , Some (Duration :: from_millis (234000))) ; assert_eq ! (pipe . server . idle_timeout () , Some (Duration :: from_millis (234000))) ; }
};
}
