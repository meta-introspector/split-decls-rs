// Generated macro for bbr_enter_startup (function)
macro_rules! Depcrate_recovery_congestion_bbr_initbbr_enter_startup {
() => {
// Module: crate::recovery::congestion::bbr::init
// Provides: {"bbr_enter_startup"}
// Dependencies: {}
pub fn bbr_enter_startup (r : & mut Congestion) { let bbr = & mut r . bbr_state ; bbr . state = BBRStateMachine :: Startup ; bbr . pacing_gain = BBR_HIGH_GAIN ; bbr . cwnd_gain = BBR_HIGH_GAIN ; }
};
}
