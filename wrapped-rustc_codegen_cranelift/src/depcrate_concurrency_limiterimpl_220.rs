// Generated macro for impl_220 (impl)
macro_rules! Depcrate_concurrency_limiterimpl_220 {
() => {
// Module: crate::concurrency_limiter
// Provides: {"impl_220"}
// Dependencies: {}
impl Drop for ConcurrencyLimiterToken { fn drop (& mut self) { let mut state = self . state . lock () . unwrap () ; state . job_finished () ; self . available_token_condvar . notify_one () ; } }
};
}
