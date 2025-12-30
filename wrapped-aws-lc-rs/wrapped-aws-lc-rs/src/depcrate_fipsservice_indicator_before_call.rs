// Generated macro for service_indicator_before_call (function)
macro_rules! Depcrate_fipsservice_indicator_before_call {
() => {
// Module: crate::fips
// Provides: {"service_indicator_before_call"}
// Dependencies: {}
# [cfg (all (feature = "fips" , debug_assertions))] # [inline] pub (crate) fn service_indicator_before_call () -> u64 { unsafe { aws_lc :: FIPS_service_indicator_before_call () } }
};
}
