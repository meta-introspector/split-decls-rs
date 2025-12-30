// Generated macro for impl_1360 (impl)
macro_rules! Depcrate_recovery_congestion_delivery_rateimpl_1360 {
() => {
// Module: crate::recovery::congestion::delivery_rate
// Provides: {"impl_1360"}
// Dependencies: {}
impl Default for Rate { fn default () -> Self { let now = Instant :: now () ; Rate { delivered : 0 , delivered_time : now , first_sent_time : now , end_of_app_limited : 0 , last_sent_packet : 0 , largest_acked : 0 , rate_sample : RateSample :: new () , } } }
};
}
