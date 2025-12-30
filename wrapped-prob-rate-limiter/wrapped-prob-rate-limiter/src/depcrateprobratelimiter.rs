// Generated macro for ProbRateLimiter (struct)
macro_rules! DepcrateProbRateLimiter {
() => {
// Module: crate
// Provides: {"ProbRateLimiter"}
// Dependencies: {}
# [doc = " A probabilistic rate-limiter."] # [doc = " - When not overloaded, accepts all requests"] # [doc = " - As load approaches limit, probabilistically rejects more and more requests."] # [doc = " - Onset of overload does not trigger a sudden total outage."] # [derive (Clone , Debug)] pub struct ProbRateLimiter { tick_duration : Duration , max_cost : u32 , cost : u32 , last : Instant , prng : Rand32 , }
};
}
