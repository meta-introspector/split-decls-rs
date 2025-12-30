// Generated macro for RateSample (struct)
macro_rules! Depcrate_recovery_congestion_delivery_rateRateSample {
() => {
// Module: crate::recovery::congestion::delivery_rate
// Provides: {"RateSample"}
// Dependencies: {}
# [derive (Debug)] struct RateSample { bandwidth : Bandwidth , is_app_limited : bool , interval : Duration , delivered : usize , prior_delivered : usize , prior_time : Option < Instant > , send_elapsed : Duration , ack_elapsed : Duration , rtt : Duration , }
};
}
