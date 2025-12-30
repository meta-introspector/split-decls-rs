// Generated macro for impl_1512 (impl)
macro_rules! Depcrate_recovery_gcongestion_bbr2impl_1512 {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"impl_1512"}
// Dependencies: {}
impl BBRv2CongestionEvent { fn new (event_time : Instant , prior_cwnd : usize , prior_bytes_in_flight : usize , is_probing_for_bandwidth : bool ,) -> Self { BBRv2CongestionEvent { event_time , prior_cwnd , prior_bytes_in_flight , is_probing_for_bandwidth , bytes_in_flight : 0 , bytes_acked : 0 , bytes_lost : 0 , end_of_round_trip : false , last_packet_send_state : Default :: default () , sample_max_bandwidth : None , sample_min_rtt : None , } } }
};
}
