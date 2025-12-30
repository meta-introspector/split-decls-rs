// Generated macro for bbr2_check_probe_rtt_done (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_probe_rtt_done {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_probe_rtt_done"}
// Dependencies: {}
pub fn bbr2_check_probe_rtt_done (r : & mut Congestion , now : Instant) { let bbr = & mut r . bbr2_state ; if let Some (probe_rtt_done_stamp) = bbr . probe_rtt_done_stamp { if now > probe_rtt_done_stamp { bbr . probe_rtt_min_stamp = now ; bbr2_restore_cwnd (r) ; bbr2_exit_probe_rtt (r , now) ; } } }
};
}
