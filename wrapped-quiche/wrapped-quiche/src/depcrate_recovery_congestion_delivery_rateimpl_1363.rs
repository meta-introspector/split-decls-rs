// Generated macro for impl_1363 (impl)
macro_rules! Depcrate_recovery_congestion_delivery_rateimpl_1363 {
() => {
// Module: crate::recovery::congestion::delivery_rate
// Provides: {"impl_1363"}
// Dependencies: {}
impl RateSample { const fn new () -> Self { RateSample { bandwidth : Bandwidth :: zero () , is_app_limited : false , interval : Duration :: ZERO , delivered : 0 , prior_delivered : 0 , prior_time : None , send_elapsed : Duration :: ZERO , ack_elapsed : Duration :: ZERO , rtt : Duration :: ZERO , } } }
};
}
