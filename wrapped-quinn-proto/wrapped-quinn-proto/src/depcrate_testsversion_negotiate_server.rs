// Generated macro for version_negotiate_server (function)
macro_rules! Depcrate_testsversion_negotiate_server {
() => {
// Module: crate::tests
// Provides: {"version_negotiate_server"}
// Dependencies: {}
# [test] fn version_negotiate_server () { let _guard = subscribe () ; let client_addr = "[::2]:7890" . parse () . unwrap () ; let mut server = Endpoint :: new (Default :: default () , Some (Arc :: new (server_config ())) , true , None ,) ; let now = Instant :: now () ; let mut buf = Vec :: with_capacity (server . config () . get_max_udp_payload_size () as usize) ; let event = server . handle (now , client_addr , None , None , hex ! ("80 0a1a2a3a 04 00000000 04 00000000 00") [..] . into () , & mut buf ,) ; let Some (DatagramEvent :: Response (Transmit { .. })) = event else { panic ! ("expected a response") ; } ; assert_ne ! (buf [0] & 0x80 , 0) ; assert_eq ! (& buf [1 .. 15] , hex ! ("00000000 04 00000000 04 00000000")) ; assert ! (buf [15 ..] . chunks (4) . any (| x | { DEFAULT_SUPPORTED_VERSIONS . contains (& u32 :: from_be_bytes (x . try_into () . unwrap ())) })) ; }
};
}
