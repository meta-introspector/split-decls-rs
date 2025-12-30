// Generated macro for preferred_address (function)
macro_rules! Depcrate_testspreferred_address {
() => {
// Module: crate::tests
// Provides: {"preferred_address"}
// Dependencies: {}
# [doc = " Ensure that a connection can be made when a preferred address is advertised by the server,"] # [doc = " regardless of whether the address is actually used."] # [test] fn preferred_address () { let _guard = subscribe () ; let mut server_config = server_config () ; server_config . preferred_address_v6 (Some ("[::1]:65535" . parse () . unwrap ())) ; let mut pair = Pair :: new (Arc :: new (EndpointConfig :: default ()) , server_config) ; pair . connect () ; }
};
}
