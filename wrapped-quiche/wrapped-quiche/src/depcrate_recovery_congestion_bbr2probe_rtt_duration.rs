// Generated macro for PROBE_RTT_DURATION (const)
macro_rules! Depcrate_recovery_congestion_bbr2PROBE_RTT_DURATION {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"PROBE_RTT_DURATION"}
// Dependencies: {}
# [doc = " A constant specifying the minimum duration for which ProbeRTT state holds"] # [doc = " inflight to BBRMinPipeCwnd or fewer packets: 200 ms."] const PROBE_RTT_DURATION : Duration = Duration :: from_millis (200) ;
};
}
