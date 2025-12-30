// Generated macro for malformed_token_len (function)
macro_rules! Depcrate_testsmalformed_token_len {
() => {
// Module: crate::tests
// Provides: {"malformed_token_len"}
// Dependencies: {}
# [test] fn malformed_token_len () { let _guard = subscribe () ; let client_addr = "[::2]:7890" . parse () . unwrap () ; let mut server = Endpoint :: new (Default :: default () , Some (Arc :: new (server_config ())) , true , None ,) ; let mut buf = Vec :: with_capacity (server . config () . get_max_udp_payload_size () as usize) ; server . handle (Instant :: now () , client_addr , None , None , hex ! ("8900 0000 0101 0000 1b1b 841b 0000 0000 3f00") [..] . into () , & mut buf ,) ; }
};
}
