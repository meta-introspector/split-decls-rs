// Generated macro for bbr2_has_elapsed_in_phase (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_has_elapsed_in_phase {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_has_elapsed_in_phase"}
// Dependencies: {}
fn bbr2_has_elapsed_in_phase (r : & mut Congestion , interval : Duration , now : Instant ,) -> bool { now > r . bbr2_state . cycle_stamp + interval }
};
}
