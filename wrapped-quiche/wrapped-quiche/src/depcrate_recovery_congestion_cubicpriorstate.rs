// Generated macro for PriorState (struct)
macro_rules! Depcrate_recovery_congestion_cubicPriorState {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"PriorState"}
// Dependencies: {}
# [doc = " Stores the CUBIC state from before the last congestion event."] # [doc = ""] # [doc = " <https://tools.ietf.org/id/draft-ietf-tcpm-rfc8312bis-00.html#section-4.9>"] # [derive (Debug , Default)] struct PriorState { congestion_window : usize , ssthresh : usize , w_max : f64 , k : f64 , epoch_start : Option < Instant > , lost_count : usize , }
};
}
