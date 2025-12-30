// Generated macro for bbr_enter_drain (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_enter_drain {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_enter_drain"}
// Dependencies: {}
fn bbr_enter_drain (r : & mut Congestion) { let bbr = & mut r . bbr_state ; bbr . state = BBRStateMachine :: Drain ; bbr . pacing_gain = 1.0 / BBR_HIGH_GAIN ; bbr . cwnd_gain = BBR_HIGH_GAIN ; }
};
}
