// Generated macro for shuffle (function)
macro_rules! Depcrate_global_rngshuffle {
() => {
// Module: crate::global_rng
// Provides: {"shuffle"}
// Dependencies: {}
# [doc = " Shuffles a slice randomly."] # [inline] pub fn shuffle < T > (slice : & mut [T]) { with_rng (| r | r . shuffle (slice)) }
};
}
