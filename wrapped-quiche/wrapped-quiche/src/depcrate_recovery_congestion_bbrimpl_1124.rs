// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_recovery_congestion_bbrimpl_1124 {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"impl_1124"}
// Dependencies: {}
impl From < BBRStateMachine > for & 'static str { fn from (state : BBRStateMachine) -> & 'static str { match state { BBRStateMachine :: Startup => "bbr_startup" , BBRStateMachine :: Drain => "bbr_drain" , BBRStateMachine :: ProbeBW => "bbr_probe_bw" , BBRStateMachine :: ProbeRTT => "bbr_probe_rtt" , } } }
};
}
