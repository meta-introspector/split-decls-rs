// Generated macro for bbr2_update_offload_budget (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_offload_budget {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_offload_budget"}
// Dependencies: {}
fn bbr2_update_offload_budget (r : & mut Congestion) { r . bbr2_state . offload_budget = 3 * r . send_quantum ; }
};
}
