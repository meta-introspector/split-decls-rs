// Generated macro for bbr2_inflight_with_headroom (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_inflight_with_headroom {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_inflight_with_headroom"}
// Dependencies: {}
fn bbr2_inflight_with_headroom (r : & mut Congestion) -> usize { let bbr = & mut r . bbr2_state ; if bbr . inflight_hi == usize :: MAX { return usize :: MAX ; } let headroom = ((HEADROOM * bbr . inflight_hi as f64) as usize) . max (1) ; bbr . inflight_hi . saturating_sub (headroom) . max (bbr2_min_pipe_cwnd (r)) }
};
}
