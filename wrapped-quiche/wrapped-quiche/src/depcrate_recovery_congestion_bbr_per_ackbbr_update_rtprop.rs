// Generated macro for bbr_update_rtprop (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_update_rtprop {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_update_rtprop"}
// Dependencies: {}
fn bbr_update_rtprop (r : & mut Congestion , now : Instant) { let bbr = & mut r . bbr_state ; let rs_rtt = r . delivery_rate . sample_rtt () ; bbr . rtprop_expired = now > bbr . rtprop_stamp + RTPROP_FILTER_LEN ; if ! rs_rtt . is_zero () && (rs_rtt <= bbr . rtprop || bbr . rtprop_expired) { bbr . rtprop = rs_rtt ; bbr . rtprop_stamp = now ; } }
};
}
