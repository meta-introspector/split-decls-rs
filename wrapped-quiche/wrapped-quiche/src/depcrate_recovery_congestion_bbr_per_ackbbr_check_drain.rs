// Generated macro for bbr_check_drain (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_check_drain {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_check_drain"}
// Dependencies: {}
fn bbr_check_drain (r : & mut Congestion , bytes_in_flight : usize , now : Instant) { if r . bbr_state . state == BBRStateMachine :: Startup && r . bbr_state . filled_pipe { bbr_enter_drain (r) ; } if r . bbr_state . state == BBRStateMachine :: Drain && bbr_bytes_in_net (r , bytes_in_flight , now) <= bbr_inflight (r , 1.0) { bbr_enter_probe_bw (r , now) ; } }
};
}
