// Generated macro for new_rng (function)
macro_rules! Depcrate_stats_rand_utilnew_rng {
() => {
// Module: crate::stats::rand_util
// Provides: {"new_rng"}
// Dependencies: {}
pub fn new_rng () -> Rng { SEED_RAND . with (| r | { let mut r = r . borrow_mut () ; let seed = ((r . rand_u64 () as u128) << 64) | (r . rand_u64 () as u128) ; Rand64 :: new (seed) }) }
};
}
