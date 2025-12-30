// Generated macro for BTLBW_FILTER_LEN (const)
macro_rules! Depcrate_recovery_congestion_bbrBTLBW_FILTER_LEN {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"BTLBW_FILTER_LEN"}
// Dependencies: {}
# [doc = " A constant specifying the length of the BBR.BtlBw max filter window for"] # [doc = " BBR.BtlBwFilter, BtlBwFilterLen is 10 packet-timed round trips."] const BTLBW_FILTER_LEN : Duration = Duration :: from_secs (10) ;
};
}
