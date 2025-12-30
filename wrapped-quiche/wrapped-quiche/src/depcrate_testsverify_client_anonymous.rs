// Generated macro for verify_client_anonymous (function)
macro_rules! Depcrate_testsverify_client_anonymous {
() => {
// Module: crate::tests
// Provides: {"verify_client_anonymous"}
// Dependencies: {}
# [test] fn verify_client_anonymous () { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . set_initial_max_data (30) ; config . set_initial_max_stream_data_bidi_local (15) ; config . set_initial_max_stream_data_bidi_remote (15) ; config . set_initial_max_streams_bidi (3) ; config . verify_peer (true) ; let mut pipe = test_utils :: Pipe :: with_server_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; assert ! (pipe . server . peer_cert () . is_none ()) ; }
};
}
