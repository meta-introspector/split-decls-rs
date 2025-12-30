// Generated macro for bbr2_enter_startup (function)
macro_rules! Depcrate_recovery_congestion_bbr2_initbbr2_enter_startup {
() => {
// Module: crate::recovery::congestion::bbr2::init
// Provides: {"bbr2_enter_startup"}
// Dependencies: {}
pub fn bbr2_enter_startup (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . state = BBR2StateMachine :: Startup ; bbr . pacing_gain = STARTUP_PACING_GAIN ; bbr . cwnd_gain = STARTUP_CWND_GAIN ; }
};
}
