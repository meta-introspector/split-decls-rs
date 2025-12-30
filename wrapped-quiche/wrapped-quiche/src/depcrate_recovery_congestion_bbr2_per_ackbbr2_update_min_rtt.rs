// Generated macro for bbr2_update_min_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_min_rtt {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_min_rtt"}
// Dependencies: {}
fn bbr2_update_min_rtt (r : & mut Congestion , now : Instant) { let bbr = & mut r . bbr2_state ; bbr . probe_rtt_expired = now > bbr . probe_rtt_min_stamp + PROBE_RTT_INTERVAL ; let rs_rtt = r . delivery_rate . sample_rtt () ; if ! rs_rtt . is_zero () && (rs_rtt < bbr . probe_rtt_min_delay || bbr . probe_rtt_expired) { bbr . probe_rtt_min_delay = rs_rtt ; bbr . probe_rtt_min_stamp = now ; } let min_rtt_expired = now > bbr . min_rtt_stamp + rs_rtt . saturating_mul (MIN_RTT_FILTER_LEN) ; if bbr . min_rtt == r . initial_rtt || min_rtt_expired { bbr . min_rtt = rs_rtt ; bbr . min_rtt_stamp = now ; } }
};
}
