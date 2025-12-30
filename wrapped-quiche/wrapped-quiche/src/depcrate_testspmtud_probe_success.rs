// Generated macro for pmtud_probe_success (function)
macro_rules! Depcrate_testspmtud_probe_success {
() => {
// Module: crate::tests
// Provides: {"pmtud_probe_success"}
// Dependencies: {}
# [rstest] fn pmtud_probe_success (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; config . set_cc_algorithm_name (cc_algorithm_name) . unwrap () ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1"]) . unwrap () ; config . verify_peer (false) ; config . set_max_send_udp_payload_size (1400) ; config . discover_pmtu (true) ; let mut pipe = test_utils :: Pipe :: with_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . advance () , Ok (())) ; let pmtud = pipe . client . paths . get_active_mut () . unwrap () . pmtud . as_mut () . unwrap () ; assert ! (! pmtud . should_probe ()) ; let current_mtu = pmtud . get_current_mtu () ; assert_eq ! (current_mtu , 1400) ; let path_stats = pipe . client . path_stats () . next () . unwrap () ; assert_eq ! (path_stats . pmtu , current_mtu) ; }
};
}
