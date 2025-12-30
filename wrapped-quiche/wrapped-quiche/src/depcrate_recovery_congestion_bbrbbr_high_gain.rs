// Generated macro for BBR_HIGH_GAIN (const)
macro_rules! Depcrate_recovery_congestion_bbrBBR_HIGH_GAIN {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"BBR_HIGH_GAIN"}
// Dependencies: {}
# [doc = " A constant specifying the minimum gain value that will allow the sending"] # [doc = " rate to double each round (2/ln(2) ~= 2.89), used in Startup mode for both"] # [doc = " BBR.pacing_gain and BBR.cwnd_gain."] const BBR_HIGH_GAIN : f64 = 2.89 ;
};
}
