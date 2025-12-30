// Generated macro for bbr2_init (function)
macro_rules! Depcrate_recovery_congestion_bbr2_initbbr2_init {
() => {
// Module: crate::recovery::congestion::bbr2::init
// Provides: {"bbr2_init"}
// Dependencies: {}
pub fn bbr2_init (r : & mut Congestion) { let now = Instant :: now () ; let bbr = & mut r . bbr2_state ; bbr . min_rtt = r . initial_rtt ; bbr . min_rtt_stamp = now ; bbr . probe_rtt_done_stamp = None ; bbr . probe_rtt_round_done = false ; bbr . prior_cwnd = 0 ; bbr . idle_restart = false ; bbr . extra_acked_interval_start = now ; bbr . extra_acked_delivered = 0 ; bbr . bw_lo = u64 :: MAX ; bbr . bw_hi = u64 :: MAX ; bbr . inflight_lo = usize :: MAX ; bbr . inflight_hi = usize :: MAX ; bbr . probe_up_cnt = usize :: MAX ; r . send_quantum = r . max_datagram_size ; per_loss :: bbr2_reset_congestion_signals (r) ; per_loss :: bbr2_reset_lower_bounds (r) ; bbr2_init_round_counting (r) ; bbr2_init_full_pipe (r) ; pacing :: bbr2_init_pacing_rate (r) ; bbr2_enter_startup (r) ; }
};
}
