// Generated macro for sending_duplicate_scids (function)
macro_rules! Depcrate_testssending_duplicate_scids {
() => {
// Module: crate::tests
// Provides: {"sending_duplicate_scids"}
// Dependencies: {}
# [rstest] fn sending_duplicate_scids (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . load_cert_chain_from_pem_file ("examples/cert.crt") . unwrap () ; config . load_priv_key_from_pem_file ("examples/cert.key") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . verify_peer (false) ; config . set_active_connection_id_limit (3) ; let mut pipe = test_utils :: Pipe :: with_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; let (scid_1 , reset_token_1) = test_utils :: create_cid_and_reset_token (16) ; assert_eq ! (pipe . client . new_scid (& scid_1 , reset_token_1 , false) , Ok (1)) ; assert_eq ! (pipe . advance () , Ok (())) ; let reset_token_2 = reset_token_1 . wrapping_add (1) ; assert_eq ! (pipe . client . new_scid (& scid_1 , reset_token_2 , false) , Err (Error :: InvalidState) ,) ; assert_eq ! (pipe . client . new_scid (& scid_1 , reset_token_1 , false) , Ok (1)) ; assert ! (! pipe . client . ids . has_new_scids ()) ; assert_eq ! (pipe . server . retire_dcid (1) , Ok (())) ; assert_eq ! (pipe . advance () , Ok (())) ; assert_eq ! (pipe . client . new_scid (& scid_1 , reset_token_1 , false) , Ok (2)) ; }
};
}
