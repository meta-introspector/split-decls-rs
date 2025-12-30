// Generated macro for bbr2_update_ack_aggregation (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_ack_aggregation {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_ack_aggregation"}
// Dependencies: {}
fn bbr2_update_ack_aggregation (r : & mut Congestion , packet : & Acked , now : Instant) { let bbr = & mut r . bbr2_state ; let interval = now - bbr . extra_acked_interval_start ; let mut expected_delivered = (bbr . bw as f64 * interval . as_secs_f64 ()) as usize ; if bbr . extra_acked_delivered <= expected_delivered { bbr . extra_acked_delivered = 0 ; bbr . extra_acked_interval_start = now ; expected_delivered = 0 ; } bbr . extra_acked_delivered += packet . size ; let extra = bbr . extra_acked_delivered . saturating_sub (expected_delivered) ; let extra = extra . min (r . congestion_window) ; let extra_acked_filter_len = r . delivery_rate . sample_rtt () . saturating_mul (MIN_RTT_FILTER_LEN) ; bbr . extra_acked = bbr . extra_acked_filter . running_max (extra_acked_filter_len , bbr . start_time + Duration :: from_secs (bbr . round_count) , extra ,) ; }
};
}
