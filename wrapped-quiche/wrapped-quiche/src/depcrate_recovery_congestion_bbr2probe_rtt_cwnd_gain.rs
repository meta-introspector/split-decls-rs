// Generated macro for PROBE_RTT_CWND_GAIN (const)
macro_rules! Depcrate_recovery_congestion_bbr2PROBE_RTT_CWND_GAIN {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"PROBE_RTT_CWND_GAIN"}
// Dependencies: {}
# [doc = " A constant specifying the gain value for calculating the cwnd during"] # [doc = " ProbeRTT: 0.5 (meaning that ProbeRTT attempts to reduce in-flight data to"] # [doc = " 50% of the estimated BDP)."] const PROBE_RTT_CWND_GAIN : f64 = 0.5 ;
};
}
