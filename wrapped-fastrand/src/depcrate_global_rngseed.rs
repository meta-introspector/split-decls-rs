// Generated macro for seed (function)
macro_rules! Depcrate_global_rngseed {
() => {
// Module: crate::global_rng
// Provides: {"seed"}
// Dependencies: {}
# [doc = " Initializes the thread-local generator with the given seed."] # [inline] pub fn seed (seed : u64) { with_rng (| r | r . seed (seed)) ; }
};
}
