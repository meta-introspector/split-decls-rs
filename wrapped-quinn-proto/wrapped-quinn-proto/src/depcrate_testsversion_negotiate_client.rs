// Generated macro for version_negotiate_client (function)
macro_rules! Depcrate_testsversion_negotiate_client {
() => {
// Module: crate::tests
// Provides: {"version_negotiate_client"}
// Dependencies: {}
# [test] fn version_negotiate_client () { let _guard = subscribe () ; let server_addr = "[::2]:7890" . parse () . unwrap () ; let cid_generator_factory : fn () -> Box < dyn ConnectionIdGenerator > = | | Box :: new (RandomConnectionIdGenerator :: new (0)) ; let mut client = Endpoint :: new (Arc :: new (EndpointConfig { connection_id_generator_factory : Arc :: new (cid_generator_factory) , .. Default :: default () }) , None , true , None ,) ; let (_ , mut client_ch) = client . connect (Instant :: now () , client_config () , server_addr , "localhost") . unwrap () ; let now = Instant :: now () ; let mut buf = Vec :: with_capacity (client . config () . get_max_udp_payload_size () as usize) ; let opt_event = client . handle (now , server_addr , None , None , hex ! ("80 00000000 00 04 00000000
             0a1a2a3a") [..] . into () , & mut buf ,) ; if let Some (DatagramEvent :: ConnectionEvent (_ , event)) = opt_event { client_ch . handle_event (event) ; } assert_matches ! (client_ch . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: VersionMismatch , })) ; }
};
}
