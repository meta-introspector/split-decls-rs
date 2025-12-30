// Generated macro for impl_128 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_128 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_128"}
// Dependencies: {}
impl Serialize for ConnectionCloseDetails { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state : < S as Serializer > :: SerializeStruct = s . serialize_struct ("enriched_connection_error" , 3) ? ; if let Some (pe) = & self . peer_error { state . serialize_field ("peer_error" , & SerializableConnectionError (pe) ,) ? ; } if let Some (le) = & self . local_error { state . serialize_field ("local_error" , & SerializableConnectionError (le) ,) ? ; } state . serialize_field ("timed_out" , & self . timed_out) ? ; state . end () } }
};
}
