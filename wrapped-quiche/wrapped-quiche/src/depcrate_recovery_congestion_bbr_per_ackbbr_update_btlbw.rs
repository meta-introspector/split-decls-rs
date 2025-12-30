// Generated macro for bbr_update_btlbw (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_update_btlbw {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_update_btlbw"}
// Dependencies: {}
fn bbr_update_btlbw (r : & mut Congestion , packet : & Acked , _bytes_in_flight : usize) { bbr_update_round (r , packet) ; if r . delivery_rate () . to_bytes_per_second () >= r . bbr_state . btlbw || ! r . delivery_rate . sample_is_app_limited () { r . bbr_state . btlbw = r . bbr_state . btlbwfilter . running_max (BTLBW_FILTER_LEN , r . bbr_state . start_time + Duration :: from_secs (r . bbr_state . round_count) , r . delivery_rate () . to_bytes_per_second () ,) ; } }
};
}
