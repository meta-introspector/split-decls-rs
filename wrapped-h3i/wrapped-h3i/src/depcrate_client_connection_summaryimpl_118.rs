// Generated macro for impl_118 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_118 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_118"}
// Dependencies: {}
impl Serialize for ConnectionSummary { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = s . serialize_struct ("path_stats" , 4) ? ; state . serialize_field ("stream_map" , & self . stream_map) ? ; state . serialize_field ("stats" , & self . stats . as_ref () . map (SerializableStats) ,) ? ; let p : Vec < SerializablePathStats > = self . path_stats . iter () . map (SerializablePathStats) . collect () ; state . serialize_field ("path_stats" , & p) ? ; state . serialize_field ("error" , & self . conn_close_details) ? ; state . serialize_field ("missed_close_trigger_frames" , & self . stream_map . missing_close_trigger_frames () ,) ? ; state . end () } }
};
}
