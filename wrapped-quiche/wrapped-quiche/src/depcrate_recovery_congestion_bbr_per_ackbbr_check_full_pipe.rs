// Generated macro for bbr_check_full_pipe (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_check_full_pipe {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_check_full_pipe"}
// Dependencies: {}
fn bbr_check_full_pipe (r : & mut Congestion) { if r . bbr_state . filled_pipe || ! r . bbr_state . round_start || r . delivery_rate . sample_is_app_limited () { return ; } if r . bbr_state . btlbw >= (r . bbr_state . full_bw as f64 * BTLBW_GROWTH_TARGET) as u64 { r . bbr_state . full_bw = r . bbr_state . btlbw ; r . bbr_state . full_bw_count = 0 ; return ; } r . bbr_state . full_bw_count += 1 ; if r . bbr_state . full_bw_count >= 3 { r . bbr_state . filled_pipe = true ; } }
};
}
