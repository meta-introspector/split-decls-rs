// Generated macro for get_sentinel_max_time (function)
macro_rules! Depcrate_ocspget_sentinel_max_time {
() => {
// Module: crate::ocsp
// Provides: {"get_sentinel_max_time"}
// Dependencies: {}
fn get_sentinel_max_time () -> & 'static Asn1GeneralizedTimeRef { SENTINEL_MAX_TIME . get_or_init (| | { Asn1GeneralizedTime :: from_str ("99991231235959Z") . expect ("Failed to create sentinel time") }) . as_ref () }
};
}
