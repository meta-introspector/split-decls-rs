// Generated macro for ConcurrencyLimiterToken (struct)
macro_rules! Depcrate_concurrency_limiterConcurrencyLimiterToken {
() => {
// Module: crate::concurrency_limiter
// Provides: {"ConcurrencyLimiterToken"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct ConcurrencyLimiterToken { state : Arc < Mutex < state :: ConcurrencyLimiterState > > , available_token_condvar : Arc < Condvar > , }
};
}
