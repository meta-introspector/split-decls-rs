// Generated macro for bbr_check_probe_rtt (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_check_probe_rtt {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_check_probe_rtt"}
// Dependencies: {}
fn bbr_check_probe_rtt (r : & mut Congestion , bytes_in_flight : usize , now : Instant) { if r . bbr_state . state != BBRStateMachine :: ProbeRTT && r . bbr_state . rtprop_expired && ! r . bbr_state . idle_restart { bbr_enter_probe_rtt (r) ; r . bbr_state . prior_cwnd = bbr_save_cwnd (r) ; r . bbr_state . probe_rtt_done_stamp = None ; } if r . bbr_state . state == BBRStateMachine :: ProbeRTT { bbr_handle_probe_rtt (r , bytes_in_flight , now) ; } r . bbr_state . idle_restart = false ; }
};
}
