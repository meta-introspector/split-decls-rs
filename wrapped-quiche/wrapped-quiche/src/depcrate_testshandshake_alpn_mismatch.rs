// Generated macro for handshake_alpn_mismatch (function)
macro_rules! Depcrate_testshandshake_alpn_mismatch {
() => {
// Module: crate::tests
// Provides: {"handshake_alpn_mismatch"}
// Dependencies: {}
# [rstest] fn handshake_alpn_mismatch (# [values ("cubic" , "bbr2" , "bbr2_gcongestion")] cc_algorithm_name : & str ,) { let mut buf = [0 ; 65535] ; let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; assert_eq ! (config . set_cc_algorithm_name (cc_algorithm_name) , Ok (())) ; config . set_application_protos (& [b"proto3\x06proto4"]) . unwrap () ; config . verify_peer (false) ; let mut pipe = test_utils :: Pipe :: with_client_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Err (Error :: TlsFail)) ; assert_eq ! (pipe . client . application_proto () , b"") ; assert_eq ! (pipe . server . application_proto () , b"") ; let (len , _) = pipe . server . send (& mut buf) . unwrap () ; assert_eq ! (len , 1200) ; assert_eq ! (pipe . server . send (& mut buf) , Err (Error :: Done)) ; assert_eq ! (pipe . server . sent_count , 1) ; }
};
}
