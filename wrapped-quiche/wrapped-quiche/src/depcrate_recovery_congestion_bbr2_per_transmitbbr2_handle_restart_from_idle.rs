// Generated macro for bbr2_handle_restart_from_idle (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_transmitbbr2_handle_restart_from_idle {
() => {
// Module: crate::recovery::congestion::bbr2::per_transmit
// Provides: {"bbr2_handle_restart_from_idle"}
// Dependencies: {}
fn bbr2_handle_restart_from_idle (r : & mut Congestion , bytes_in_flight : usize , now : Instant ,) { if bytes_in_flight == 0 && r . delivery_rate . app_limited () { r . bbr2_state . idle_restart = true ; r . bbr2_state . extra_acked_interval_start = now ; if per_ack :: bbr2_is_in_a_probe_bw_state (r) { pacing :: bbr2_set_pacing_rate_with_gain (r , 1.0) ; } else if r . bbr2_state . state == BBR2StateMachine :: ProbeRTT { per_ack :: bbr2_check_probe_rtt_done (r , now) ; } } }
};
}
