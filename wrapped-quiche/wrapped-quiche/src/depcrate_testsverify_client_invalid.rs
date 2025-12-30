// Generated macro for verify_client_invalid (function)
macro_rules! Depcrate_testsverify_client_invalid {
() => {
// Module: crate::tests
// Provides: {"verify_client_invalid"}
// Dependencies: {}
# [cfg (not (feature = "openssl"))] # [test] fn verify_client_invalid () { let mut server_config = Config :: new (PROTOCOL_VERSION) . unwrap () ; server_config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; server_config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; server_config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; server_config . set_initial_max_data (30) ; server_config . set_initial_max_stream_data_bidi_local (15) ; server_config . set_initial_max_stream_data_bidi_remote (15) ; server_config . set_initial_max_streams_bidi (3) ; server_config . verify_peer (true) ; let mut client_config = Config :: new (PROTOCOL_VERSION) . unwrap () ; client_config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; client_config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; client_config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; client_config . set_initial_max_data (30) ; client_config . set_initial_max_stream_data_bidi_local (15) ; client_config . set_initial_max_stream_data_bidi_remote (15) ; client_config . set_initial_max_streams_bidi (3) ; client_config . load_verify_locations_from_file ("examples/rootca.crt") . unwrap () ; client_config . verify_peer (true) ; let mut pipe = test_utils :: Pipe :: with_client_and_server_config (& mut client_config , & mut server_config ,) . unwrap () ; assert_eq ! (pipe . handshake () , Err (Error :: TlsFail)) ; assert ! (pipe . server . peer_cert () . is_some ()) ; }
};
}
