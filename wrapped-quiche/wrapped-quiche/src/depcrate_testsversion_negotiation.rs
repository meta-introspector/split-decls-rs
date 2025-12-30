// Generated macro for version_negotiation (function)
macro_rules! Depcrate_testsversion_negotiation {
() => {
// Module: crate::tests
// Provides: {"version_negotiation"}
// Dependencies: {}
# [test] fn version_negotiation () { let mut buf = [0 ; 65535] ; let mut config = Config :: new (0xbabababa) . unwrap () ; config . set_application_protos (& [b"proto1" , b"proto2"]) . unwrap () ; config . verify_peer (false) ; let mut pipe = test_utils :: Pipe :: with_client_config (& mut config) . unwrap () ; let (mut len , _) = pipe . client . send (& mut buf) . unwrap () ; let hdr = Header :: from_slice (& mut buf [.. len] , 0) . unwrap () ; len = negotiate_version (& hdr . scid , & hdr . dcid , & mut buf) . unwrap () ; assert_eq ! (pipe . client_recv (& mut buf [.. len]) , Ok (len)) ; assert_eq ! (pipe . handshake () , Ok (())) ; assert_eq ! (pipe . client . version , PROTOCOL_VERSION) ; assert_eq ! (pipe . server . version , PROTOCOL_VERSION) ; }
};
}
