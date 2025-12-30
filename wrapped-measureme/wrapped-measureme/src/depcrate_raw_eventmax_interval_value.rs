// Generated macro for MAX_INTERVAL_VALUE (const)
macro_rules! Depcrate_raw_eventMAX_INTERVAL_VALUE {
() => {
// Module: crate::raw_event
// Provides: {"MAX_INTERVAL_VALUE"}
// Dependencies: {}
# [doc = " The max value we can represent with the 48 bits available."] # [doc = " The highest two values are reserved for the `INSTANT_MARKER` and `INTEGER_MARKER`."] pub const MAX_INTERVAL_VALUE : u64 = INTEGER_MARKER - 1 ;
};
}
