// Generated macro for reject_short_idcid (function)
macro_rules! Depcrate_testsreject_short_idcid {
() => {
// Module: crate::tests
// Provides: {"reject_short_idcid"}
// Dependencies: {}
# [test] fn reject_short_idcid () { let _guard = subscribe () ; let client_addr = "[::2]:7890" . parse () . unwrap () ; let mut server = Endpoint :: new (Default :: default () , Some (Arc :: new (server_config ())) , true , None ,) ; let now = Instant :: now () ; let mut buf = Vec :: with_capacity (server . config () . get_max_udp_payload_size () as usize) ; let mut initial = BytesMut :: from (hex ! ("c4 00000001 00 00 00 3f") . as_ref ()) ; initial . resize (MIN_INITIAL_SIZE . into () , 0) ; let event = server . handle (now , client_addr , None , None , initial , & mut buf) ; let Some (DatagramEvent :: Response (Transmit { .. })) = event else { panic ! ("expected an initial close") ; } ; }
};
}
