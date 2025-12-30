// Generated macro for impl_134 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_134 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_134"}
// Dependencies: {}
impl Serialize for SerializableConnectionError < '_ > { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = s . serialize_struct ("path_stats" , 3) ? ; state . serialize_field ("is_app" , & self . 0 . is_app) ? ; state . serialize_field ("error_code" , & self . 0 . error_code) ? ; let max = cmp :: min (self . 0 . reason . len () , MAX_SERIALIZED_BUFFER_LEN) ; state . serialize_field ("reason" , & String :: from_utf8_lossy (& self . 0 . reason [.. max]) ,) ? ; state . end () } }
};
}
