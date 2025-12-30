// Generated macro for Time (enum)
macro_rules! Depcrate_common_timeTime {
() => {
// Module: crate::common::time
// Provides: {"Time"}
// Dependencies: {}
# [doc = " A user-provided timer to time background tasks."] # [derive (Clone)] pub (crate) enum Time { Timer (Arc < dyn Timer + Send + Sync >) , Empty , }
};
}
