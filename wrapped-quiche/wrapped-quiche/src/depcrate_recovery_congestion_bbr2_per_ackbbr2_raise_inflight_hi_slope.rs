// Generated macro for bbr2_raise_inflight_hi_slope (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_raise_inflight_hi_slope {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_raise_inflight_hi_slope"}
// Dependencies: {}
fn bbr2_raise_inflight_hi_slope (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; let growth_this_round = (1 << bbr . bw_probe_up_rounds) * r . max_datagram_size ; bbr . bw_probe_up_rounds = (bbr . bw_probe_up_rounds + 1) . min (30) ; bbr . probe_up_cnt = (r . congestion_window / growth_this_round) . max (1) ; }
};
}
