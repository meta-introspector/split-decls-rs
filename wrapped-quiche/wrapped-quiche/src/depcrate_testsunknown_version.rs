// Generated macro for unknown_version (function)
macro_rules! Depcrate_testsunknown_version {
() => {
// Module: crate::tests
// Provides: {"unknown_version"}
// Dependencies: {}
# [test] fn unknown_version () { let mut config = Config :: new (0xbabababa) . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . verify_peer (false) ; let mut pipe = test_utils :: Pipe :: with_client_config (& mut config) . unwrap () ; assert_eq ! (pipe . handshake () , Err (Error :: UnknownVersion)) ; }
};
}
