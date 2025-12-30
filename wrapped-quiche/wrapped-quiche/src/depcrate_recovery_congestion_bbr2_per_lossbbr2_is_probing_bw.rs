// Generated macro for bbr2_is_probing_bw (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_is_probing_bw {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_is_probing_bw"}
// Dependencies: {}
fn bbr2_is_probing_bw (r : & mut Congestion) -> bool { let state = r . bbr2_state . state ; state == BBR2StateMachine :: Startup || state == BBR2StateMachine :: ProbeBWREFILL || state == BBR2StateMachine :: ProbeBWUP }
};
}
