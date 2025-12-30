// Generated macro for bbr2_probe_inflight_hi_upward (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_probe_inflight_hi_upward {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_probe_inflight_hi_upward"}
// Dependencies: {}
fn bbr2_probe_inflight_hi_upward (r : & mut Congestion) { if r . app_limited || r . congestion_window < r . bbr2_state . inflight_hi { return ; } let bbr = & mut r . bbr2_state ; bbr . bw_probe_up_acks += 1 ; if bbr . bw_probe_up_acks >= bbr . probe_up_cnt { let delta = bbr . bw_probe_up_acks / bbr . probe_up_cnt ; bbr . bw_probe_up_acks -= delta * bbr . probe_up_cnt ; bbr . inflight_hi += delta * r . max_datagram_size ; } if bbr . round_start { bbr2_raise_inflight_hi_slope (r) ; } }
};
}
