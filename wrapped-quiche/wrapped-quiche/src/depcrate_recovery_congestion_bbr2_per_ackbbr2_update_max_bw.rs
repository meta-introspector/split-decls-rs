// Generated macro for bbr2_update_max_bw (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_max_bw {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_max_bw"}
// Dependencies: {}
pub fn bbr2_update_max_bw (r : & mut Congestion , packet : & Acked) { bbr2_update_round (r , packet) ; if r . delivery_rate () . to_bytes_per_second () >= r . bbr2_state . max_bw || ! r . delivery_rate . sample_is_app_limited () { let max_bw_filter_len = r . delivery_rate . sample_rtt () . saturating_mul (MIN_RTT_FILTER_LEN) ; r . bbr2_state . max_bw = r . bbr2_state . max_bw_filter . running_max (max_bw_filter_len , r . bbr2_state . start_time + Duration :: from_secs (r . bbr2_state . cycle_count) , r . delivery_rate () . to_bytes_per_second () ,) ; } }
};
}
