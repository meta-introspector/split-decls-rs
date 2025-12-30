// Generated macro for bbr2_check_startup_full_bandwidth (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_startup_full_bandwidth {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_startup_full_bandwidth"}
// Dependencies: {}
fn bbr2_check_startup_full_bandwidth (r : & mut Congestion) { if r . bbr2_state . filled_pipe || ! r . bbr2_state . round_start || r . delivery_rate . sample_is_app_limited () { return ; } if r . bbr2_state . max_bw >= (r . bbr2_state . full_bw as f64 * MAX_BW_GROWTH_THRESHOLD) as u64 { r . bbr2_state . full_bw = r . bbr2_state . max_bw ; r . bbr2_state . full_bw_count = 0 ; return ; } r . bbr2_state . full_bw_count += 1 ; if r . bbr2_state . full_bw_count >= MAX_BW_COUNT { r . bbr2_state . filled_pipe = true ; } }
};
}
