// Generated macro for transport_params_forbid_duplicates (function)
macro_rules! Depcrate_teststransport_params_forbid_duplicates {
() => {
// Module: crate::tests
// Provides: {"transport_params_forbid_duplicates"}
// Dependencies: {}
# [test] fn transport_params_forbid_duplicates () { let initial_source_connection_id = b"id" ; let initial_source_connection_id_raw = [15 , initial_source_connection_id . len () as u8 , initial_source_connection_id [0] , initial_source_connection_id [1] ,] ; let tp = TransportParams :: decode (initial_source_connection_id_raw . as_slice () , true , None ,) . unwrap () ; assert_eq ! (tp . initial_source_connection_id , Some (initial_source_connection_id . to_vec () . into ())) ; let mut raw_params = Vec :: new () ; raw_params . append (& mut initial_source_connection_id_raw . to_vec ()) ; raw_params . append (& mut initial_source_connection_id_raw . to_vec ()) ; assert_eq ! (TransportParams :: decode (raw_params . as_slice () , true , None) , Err (Error :: InvalidTransportParam)) ; }
};
}
