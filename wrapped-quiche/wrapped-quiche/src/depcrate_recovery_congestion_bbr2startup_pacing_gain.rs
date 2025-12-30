// Generated macro for STARTUP_PACING_GAIN (const)
macro_rules! Depcrate_recovery_congestion_bbr2STARTUP_PACING_GAIN {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"STARTUP_PACING_GAIN"}
// Dependencies: {}
# [doc = " A constant specifying the minimum gain value"] # [doc = " for calculating the pacing rate that will allow the sending rate to"] # [doc = " double each round (4*ln(2) ~=2.77 ) BBRStartupPacingGain; used in"] # [doc = " Startup mode for BBR.pacing_gain."] const STARTUP_PACING_GAIN : f64 = 2.77 ;
};
}
