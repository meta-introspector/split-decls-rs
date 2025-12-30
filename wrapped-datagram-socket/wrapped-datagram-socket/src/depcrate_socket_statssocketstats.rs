// Generated macro for SocketStats (struct)
macro_rules! Depcrate_socket_statsSocketStats {
() => {
// Module: crate::socket_stats
// Provides: {"SocketStats"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , Default)] pub struct SocketStats { pub pmtu : u16 , pub rtt_us : i64 , pub min_rtt_us : i64 , pub max_rtt_us : i64 , pub rtt_var_us : i64 , pub cwnd : u64 , pub total_pto_count : u64 , pub packets_sent : u64 , pub packets_recvd : u64 , pub packets_lost : u64 , pub packets_lost_spurious : u64 , pub packets_retrans : u64 , pub bytes_sent : u64 , pub bytes_recvd : u64 , pub bytes_lost : u64 , pub bytes_retrans : u64 , pub bytes_unsent : u64 , pub delivery_rate : u64 , pub max_bandwidth : Option < u64 > , pub startup_exit : Option < StartupExit > , pub bytes_in_flight_duration_us : u64 , }
};
}
