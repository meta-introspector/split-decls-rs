// Generated macro for RandomSource (trait)
macro_rules! Depcrate_random_stateRandomSource {
() => {
// Module: crate::random_state
// Provides: {"RandomSource"}
// Dependencies: {}
# [doc = " A supplier of Randomness used for different hashers."] # [doc = " See [set_random_source]."] # [doc = ""] # [doc = " If [set_random_source] aHash will default to the best available source of randomness."] # [doc = " In order this is:"] # [doc = " 1. OS provided random number generator (available if the `runtime-rng` flag is enabled which it is by default) - This should be very strong."] # [doc = " 2. Strong compile time random numbers used to permute a static \"counter\". (available if `compile-time-rng` is enabled."] # [doc = " __Enabling this is recommended if `runtime-rng` is not possible__)"] # [doc = " 3. A static counter that adds the memory address of each [RandomState] created permuted with fixed constants."] # [doc = " (Similar to above but with fixed keys) - This is the weakest option. The strength of this heavily depends on whether or not ASLR is enabled."] # [doc = " (Rust enables ASLR by default)"] pub trait RandomSource { fn gen_hasher_seed (& self) -> usize ; }
};
}
