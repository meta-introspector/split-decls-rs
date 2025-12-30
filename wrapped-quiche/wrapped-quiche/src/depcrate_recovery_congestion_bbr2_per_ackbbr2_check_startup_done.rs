// Generated macro for bbr2_check_startup_done (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_startup_done {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_startup_done"}
// Dependencies: {}
fn bbr2_check_startup_done (r : & mut Congestion) { bbr2_check_startup_full_bandwidth (r) ; bbr2_check_startup_high_loss (r) ; if r . bbr2_state . state == BBR2StateMachine :: Startup && r . bbr2_state . filled_pipe { bbr2_enter_drain (r) ; } }
};
}
