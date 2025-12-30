// Generated macro for MAX_BURST_INTERVAL (const)
macro_rules! Depcrate_connection_pacingMAX_BURST_INTERVAL {
() => {
// Module: crate::connection::pacing
// Provides: {"MAX_BURST_INTERVAL"}
// Dependencies: {}
# [doc = " Maximum period of traffic to batch together on a slow connection"] # [doc = ""] # [doc = " Takes precedence over [`MIN_BURST_SIZE`]."] const MAX_BURST_INTERVAL : Duration = Duration :: from_millis (10) ;
};
}
