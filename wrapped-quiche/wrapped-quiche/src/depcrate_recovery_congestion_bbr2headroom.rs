// Generated macro for HEADROOM (const)
macro_rules! Depcrate_recovery_congestion_bbr2HEADROOM {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"HEADROOM"}
// Dependencies: {}
# [doc = " The multiplicative factor to apply to BBR.inflight_hi"] # [doc = " when attempting to leave free headroom in the path (e.g. free space"] # [doc = " in the bottleneck buffer or free time slots in the bottleneck link)"] # [doc = " that can be used by cross traffic (the value is 0.85)."] const HEADROOM : f64 = 0.85 ;
};
}
