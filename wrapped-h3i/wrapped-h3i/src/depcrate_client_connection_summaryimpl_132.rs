// Generated macro for impl_132 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_132 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_132"}
// Dependencies: {}
impl Serialize for SerializableStats < '_ > { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = s . serialize_struct ("path_stats" , 14) ? ; state . serialize_field ("recv" , & self . 0 . recv) ? ; state . serialize_field ("sent" , & self . 0 . sent) ? ; state . serialize_field ("lost" , & self . 0 . lost) ? ; state . serialize_field ("retrans" , & self . 0 . retrans) ? ; state . serialize_field ("sent_bytes" , & self . 0 . sent_bytes) ? ; state . serialize_field ("recv_bytes" , & self . 0 . recv_bytes) ? ; state . serialize_field ("lost_bytes" , & self . 0 . lost_bytes) ? ; state . serialize_field ("stream_retrans_bytes" , & self . 0 . stream_retrans_bytes ,) ? ; state . serialize_field ("paths_count" , & self . 0 . paths_count) ? ; state . serialize_field ("reset_stream_count_local" , & self . 0 . reset_stream_count_local ,) ? ; state . serialize_field ("stopped_stream_count_local" , & self . 0 . stopped_stream_count_local ,) ? ; state . serialize_field ("reset_stream_count_remote" , & self . 0 . reset_stream_count_remote ,) ? ; state . serialize_field ("stopped_stream_count_remote" , & self . 0 . stopped_stream_count_remote ,) ? ; state . serialize_field ("path_challenge_rx_count" , & self . 0 . path_challenge_rx_count ,) ? ; state . end () } }
};
}
