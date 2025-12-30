// Generated macro for new_fair_ip_address_rate_limiter (function)
macro_rules! Depcratenew_fair_ip_address_rate_limiter {
() => {
// Module: crate
// Provides: {"new_fair_ip_address_rate_limiter"}
// Dependencies: {}
# [doc = " Creates a new fair rate limiter for IP addresses."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error when `max_cost_per_sec` is less than 1.0."] pub fn new_fair_ip_address_rate_limiter (max_cost_per_sec : f32 ,) -> Result < FairRateLimiter < IpAddrKey , 1000 > , String > { # [allow (clippy :: cast_possible_truncation , clippy :: cast_sign_loss)] let other_max = max ((max_cost_per_sec * 0.20) as u32 , 1) ; # [allow (clippy :: cast_possible_truncation , clippy :: cast_sign_loss)] let sources_max = (max_cost_per_sec as u32) . saturating_sub (other_max) ; if max_cost_per_sec != 0.0 && sources_max == 0 { return Err (format ! ("max_cost_per_sec is too small: {max_cost_per_sec:?}")) ; } FairRateLimiter :: new (Duration :: from_secs (1) , sources_max , other_max , Rand32 :: new (0) , Instant :: now () ,) }
};
}
