// Generated macro for bbr_check_cycle_phase (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_check_cycle_phase {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_check_cycle_phase"}
// Dependencies: {}
fn bbr_check_cycle_phase (r : & mut Congestion , now : Instant) { let bbr = & mut r . bbr_state ; if bbr . state == BBRStateMachine :: ProbeBW && bbr_is_next_cycle_phase (r , now) { bbr_advance_cycle_phase (r , now) ; } }
};
}
