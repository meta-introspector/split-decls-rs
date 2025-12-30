// Generated macro for verify_custom_root (function)
macro_rules! Depcrate_testsverify_custom_root {
() => {
// Module: crate::tests
// Provides: {"verify_custom_root"}
// Dependencies: {}
# [test] fn verify_custom_root () { let mut config = Config :: new (PROTOCOL_VERSION) . unwrap () ; config . verify_peer (true) ; config . load_verify_locations_from_file ("examples/rootca.crt") . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; let mut pipe = test_utils :: Pipe :: with_client_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Ok (())) ; }
};
}
