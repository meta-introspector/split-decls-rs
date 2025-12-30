// Generated macro for ConcurrencyLimiter (struct)
macro_rules! Depcrate_concurrency_limiterConcurrencyLimiter {
() => {
// Module: crate::concurrency_limiter
// Provides: {"ConcurrencyLimiter"}
// Dependencies: {}
pub (super) struct ConcurrencyLimiter { helper_thread : Option < Mutex < HelperThread > > , state : Arc < Mutex < state :: ConcurrencyLimiterState > > , available_token_condvar : Arc < Condvar > , finished : bool , }
};
}
