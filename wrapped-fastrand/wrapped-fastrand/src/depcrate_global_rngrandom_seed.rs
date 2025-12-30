// Generated macro for random_seed (function)
macro_rules! Depcrate_global_rngrandom_seed {
() => {
// Module: crate::global_rng
// Provides: {"random_seed"}
// Dependencies: {}
# [cfg (all (any (target_arch = "wasm32" , target_arch = "wasm64") , target_os = "unknown" , not (feature = "js")))] fn random_seed () -> Option < u64 > { None }
};
}
