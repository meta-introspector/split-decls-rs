// Generated macro for bbr2_enter_drain (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_enter_drain {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_enter_drain"}
// Dependencies: {}
fn bbr2_enter_drain (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . state = BBR2StateMachine :: Drain ; bbr . pacing_gain = PACING_GAIN / STARTUP_CWND_GAIN ; bbr . cwnd_gain = STARTUP_CWND_GAIN ; }
};
}
