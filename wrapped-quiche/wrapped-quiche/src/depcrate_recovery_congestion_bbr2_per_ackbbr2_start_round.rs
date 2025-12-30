// Generated macro for bbr2_start_round (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_start_round {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_start_round"}
// Dependencies: {}
fn bbr2_start_round (r : & mut Congestion) { r . bbr2_state . next_round_delivered = r . delivery_rate . delivered () ; }
};
}
