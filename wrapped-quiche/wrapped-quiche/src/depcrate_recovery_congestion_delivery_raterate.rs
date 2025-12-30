// Generated macro for Rate (struct)
macro_rules! Depcrate_recovery_congestion_delivery_rateRate {
() => {
// Module: crate::recovery::congestion::delivery_rate
// Provides: {"Rate"}
// Dependencies: {}
# [derive (Debug)] pub struct Rate { delivered : usize , delivered_time : Instant , first_sent_time : Instant , end_of_app_limited : u64 , last_sent_packet : u64 , largest_acked : u64 , rate_sample : RateSample , }
};
}
