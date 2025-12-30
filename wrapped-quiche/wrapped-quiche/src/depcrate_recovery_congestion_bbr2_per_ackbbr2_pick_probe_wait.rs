// Generated macro for bbr2_pick_probe_wait (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_pick_probe_wait {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_pick_probe_wait"}
// Dependencies: {}
fn bbr2_pick_probe_wait (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . rounds_since_probe = rand :: rand_u8 () as usize % 2 ; bbr . bw_probe_wait = Duration :: from_secs_f64 (2.0 + rand :: rand_u64_uniform (1000000) as f64 / 1000000.0 ,) ; }
};
}
