// Generated macro for client_stateless_reset (function)
macro_rules! Depcrate_testsclient_stateless_reset {
() => {
// Module: crate::tests
// Provides: {"client_stateless_reset"}
// Dependencies: {}
# [test] fn client_stateless_reset () { let _guard = subscribe () ; let mut key_material = vec ! [0 ; 64] ; let mut rng = rand :: rng () ; rng . fill_bytes (& mut key_material) ; let reset_key = hmac :: Key :: new (hmac :: HMAC_SHA256 , & key_material) ; rng . fill_bytes (& mut key_material) ; let mut endpoint_config = EndpointConfig :: new (Arc :: new (reset_key)) ; endpoint_config . cid_generator (move | | Box :: new (HashedConnectionIdGenerator :: from_key (0))) ; let endpoint_config = Arc :: new (endpoint_config) ; let mut pair = Pair :: new (endpoint_config . clone () , server_config ()) ; let (_ , server_ch) = pair . connect () ; pair . client . endpoint = Endpoint :: new (endpoint_config , Some (Arc :: new (server_config ())) , true , None) ; pair . server . connections . get_mut (& server_ch) . unwrap () . close (pair . time , VarInt (42) , (& [0xab ; 128] [..]) . into () ,) ; info ! ("resetting") ; pair . drive () ; assert_matches ! (pair . server_conn_mut (server_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: Reset })) ; }
};
}
