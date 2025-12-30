// Generated macro for failed_path_validation (function)
macro_rules! Depcrate_testsfailed_path_validation {
() => {
// Module: crate::tests
// Provides: {"failed_path_validation"}
// Dependencies: {}
# [rstest] fn failed_path_validation (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . verify_peer (false) ; config . set_active_connection_id_limit (2) ; let mut pipe = pipe_with_exchanged_cids (& mut config , 16 , 16 , 1) ; let server_addr = test_utils :: Pipe :: server_addr () ; let client_addr_2 = "127.0.0.1:5678" . parse () . unwrap () ; assert_eq ! (pipe . client . probe_path (client_addr_2 , server_addr) , Ok (1)) ; for _ in 0 .. MAX_PROBING_TIMEOUTS { test_utils :: emit_flight (& mut pipe . client) . unwrap () ; let probed_pid = pipe . client . paths . path_id_from_addrs (& (client_addr_2 , server_addr)) . unwrap () ; let probe_instant = pipe . client . paths . get (probed_pid) . unwrap () . recovery . loss_detection_timer () . unwrap () ; let timer = probe_instant . duration_since (Instant :: now ()) ; std :: thread :: sleep (timer + Duration :: from_millis (1)) ; pipe . client . on_timeout () ; } assert_eq ! (pipe . client . path_event_next () , Some (PathEvent :: FailedValidation (client_addr_2 , server_addr)) ,) ; }
};
}
