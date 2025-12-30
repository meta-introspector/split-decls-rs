// Generated macro for bbr2_quantization_budget (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_quantization_budget {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_quantization_budget"}
// Dependencies: {}
fn bbr2_quantization_budget (r : & mut Congestion , inflight : usize) -> usize { bbr2_update_offload_budget (r) ; let inflight = inflight . max (r . bbr2_state . offload_budget) ; let inflight = inflight . max (bbr2_min_pipe_cwnd (r)) ; if r . bbr2_state . state == BBR2StateMachine :: ProbeBWUP { return inflight + 2 * r . max_datagram_size ; } inflight }
};
}
