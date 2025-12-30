// Generated macro for impl_1214 (impl)
macro_rules! Depcrate_recovery_congestion_bbr2impl_1214 {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"impl_1214"}
// Dependencies: {}
impl From < BBR2StateMachine > for & 'static str { fn from (state : BBR2StateMachine) -> & 'static str { match state { BBR2StateMachine :: Startup => "bbr_startup" , BBR2StateMachine :: Drain => "bbr_drain" , BBR2StateMachine :: ProbeBWDOWN => "bbr_probe_bw_down" , BBR2StateMachine :: ProbeBWCRUISE => "bbr_probe_bw_cruise" , BBR2StateMachine :: ProbeBWREFILL => "bbr_probe_bw_refill" , BBR2StateMachine :: ProbeBWUP => "bbr_probe_bw_up" , BBR2StateMachine :: ProbeRTT => "bbr_probe_rtt" , } } }
};
}
