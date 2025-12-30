// Generated macro for bbr2_check_drain (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_drain {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_drain"}
// Dependencies: {}
fn bbr2_check_drain (r : & mut Congestion , in_flight : usize , now : Instant) { if r . bbr2_state . state == BBR2StateMachine :: Drain && in_flight <= bbr2_inflight (r , r . bbr2_state . max_bw , 1.0) { bbr2_enter_probe_bw (r , now) ; } }
};
}
