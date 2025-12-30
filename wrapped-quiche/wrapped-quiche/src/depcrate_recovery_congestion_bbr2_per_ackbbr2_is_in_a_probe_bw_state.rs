// Generated macro for bbr2_is_in_a_probe_bw_state (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_is_in_a_probe_bw_state {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_is_in_a_probe_bw_state"}
// Dependencies: {}
pub fn bbr2_is_in_a_probe_bw_state (r : & mut Congestion) -> bool { let state = r . bbr2_state . state ; state == BBR2StateMachine :: ProbeBWDOWN || state == BBR2StateMachine :: ProbeBWCRUISE || state == BBR2StateMachine :: ProbeBWREFILL || state == BBR2StateMachine :: ProbeBWUP }
};
}
