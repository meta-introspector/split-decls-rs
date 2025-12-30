// Generated macro for impl_9 (impl)
macro_rules! Depcrate_commonimpl_9 {
() => {
// Module: crate::common
// Provides: {"impl_9"}
// Dependencies: {}
# [cfg (feature = "random")] impl Default for Seed { # [doc = " Generates a random seed."] fn default () -> Self { let mut seed = [0u8 ; Seed :: BYTES] ; getrandom :: getrandom (& mut seed) . expect ("RNG failure") ; Seed (seed) } }
};
}
