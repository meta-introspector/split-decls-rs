// Generated macro for zero_length_cid (function)
macro_rules! Depcrate_testszero_length_cid {
() => {
// Module: crate::tests
// Provides: {"zero_length_cid"}
// Dependencies: {}
# [test] fn zero_length_cid () { let _guard = subscribe () ; let cid_generator_factory : fn () -> Box < dyn ConnectionIdGenerator > = | | Box :: new (RandomConnectionIdGenerator :: new (0)) ; let mut pair = Pair :: new (Arc :: new (EndpointConfig { connection_id_generator_factory : Arc :: new (cid_generator_factory) , .. EndpointConfig :: default () }) , server_config () ,) ; let (client_ch , server_ch) = pair . connect () ; info ! ("closing") ; pair . client . connections . get_mut (& client_ch) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . drive () ; pair . server . connections . get_mut (& server_ch) . unwrap () . close (pair . time , VarInt (42) , Bytes :: new ()) ; pair . connect () ; }
};
}
