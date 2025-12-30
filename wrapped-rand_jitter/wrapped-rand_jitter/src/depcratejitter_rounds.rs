// Generated macro for JITTER_ROUNDS (static)
macro_rules! DepcrateJITTER_ROUNDS {
() => {
// Module: crate
// Provides: {"JITTER_ROUNDS"}
// Dependencies: {}
# [cfg (all (feature = "std" , not (target_arch = "wasm32")))] static JITTER_ROUNDS : AtomicUsize = AtomicUsize :: new (0) ;
};
}
