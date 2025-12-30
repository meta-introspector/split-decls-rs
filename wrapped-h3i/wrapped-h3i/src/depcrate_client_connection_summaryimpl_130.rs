// Generated macro for impl_130 (impl)
macro_rules! Depcrate_client_connection_summaryimpl_130 {
() => {
// Module: crate::client::connection_summary
// Provides: {"impl_130"}
// Dependencies: {}
impl Serialize for SerializablePathStats < '_ > { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = s . serialize_struct ("path_stats" , 17) ? ; state . serialize_field ("local_addr" , & self . 0 . local_addr) ? ; state . serialize_field ("peer_addr" , & self . 0 . peer_addr) ? ; state . serialize_field ("active" , & self . 0 . active) ? ; state . serialize_field ("recv" , & self . 0 . recv) ? ; state . serialize_field ("sent" , & self . 0 . sent) ? ; state . serialize_field ("lost" , & self . 0 . lost) ? ; state . serialize_field ("retrans" , & self . 0 . retrans) ? ; state . serialize_field ("rtt" , & self . 0 . rtt . as_secs_f64 ()) ? ; state . serialize_field ("min_rtt" , & self . 0 . min_rtt . map (| x | x . as_secs_f64 ()) ,) ? ; state . serialize_field ("rttvar" , & self . 0 . rttvar . as_secs_f64 ()) ? ; state . serialize_field ("cwnd" , & self . 0 . cwnd) ? ; state . serialize_field ("sent_bytes" , & self . 0 . sent_bytes) ? ; state . serialize_field ("recv_bytes" , & self . 0 . recv_bytes) ? ; state . serialize_field ("lost_bytes" , & self . 0 . lost_bytes) ? ; state . serialize_field ("stream_retrans_bytes" , & self . 0 . stream_retrans_bytes ,) ? ; state . serialize_field ("pmtu" , & self . 0 . pmtu) ? ; state . serialize_field ("delivery_rate" , & self . 0 . delivery_rate) ? ; state . end () } }
};
}
