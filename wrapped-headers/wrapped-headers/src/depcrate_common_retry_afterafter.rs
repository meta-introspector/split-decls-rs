// Generated macro for After (enum)
macro_rules! Depcrate_common_retry_afterAfter {
() => {
// Module: crate::common::retry_after
// Provides: {"After"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] enum After { # [doc = " Retry after the given DateTime"] DateTime (HttpDate) , # [doc = " Retry after this duration has elapsed"] Delay (Seconds) , }
};
}
