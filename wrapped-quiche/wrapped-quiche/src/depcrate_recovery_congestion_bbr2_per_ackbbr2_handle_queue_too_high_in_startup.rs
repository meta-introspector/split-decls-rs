// Generated macro for bbr2_handle_queue_too_high_in_startup (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_handle_queue_too_high_in_startup {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_handle_queue_too_high_in_startup"}
// Dependencies: {}
fn bbr2_handle_queue_too_high_in_startup (r : & mut Congestion) { r . bbr2_state . filled_pipe = true ; r . bbr2_state . inflight_hi = bbr2_inflight (r , r . bbr2_state . max_bw , 1.0) ; }
};
}
