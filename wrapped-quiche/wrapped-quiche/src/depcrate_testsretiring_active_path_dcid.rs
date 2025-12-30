// Generated macro for retiring_active_path_dcid (function)
macro_rules! Depcrate_testsretiring_active_path_dcid {
() => {
// Module: crate::tests
// Provides: {"retiring_active_path_dcid"}
// Dependencies: {}
# [rstest] fn retiring_active_path_dcid (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . verify_peer (false) ; config . set_active_connection_id_limit (2) ; let mut pipe = pipe_with_exchanged_cids (& mut config , 16 , 16 , 1) ; let server_addr = test_utils :: Pipe :: server_addr () ; let client_addr_2 = "127.0.0.1:5678" . parse () . unwrap () ; assert_eq ! (pipe . client . probe_path (client_addr_2 , server_addr) , Ok (1)) ; assert_eq ! (pipe . client . retire_dcid (0) , Err (Error :: OutOfIdentifiers)) ; }
};
}
