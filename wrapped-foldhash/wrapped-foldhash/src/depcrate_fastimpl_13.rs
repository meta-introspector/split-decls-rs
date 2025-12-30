// Generated macro for impl_13 (impl)
macro_rules! Depcrate_fastimpl_13 {
() => {
// Module: crate::fast
// Provides: {"impl_13"}
// Dependencies: {}
impl SeedableRandomState { # [doc = " Generates a random [`SeedableRandomState`], similar to [`RandomState`]."] # [inline (always)] pub fn random () -> Self { Self { per_hasher_seed : gen_per_hasher_seed () , shared_seed : SharedSeed :: global_random () , } } # [doc = " Generates a fixed [`SeedableRandomState`], similar to [`FixedState`]."] # [inline (always)] pub fn fixed () -> Self { Self { per_hasher_seed : ARBITRARY3 , shared_seed : SharedSeed :: global_fixed () , } } # [doc = " Generates a [`SeedableRandomState`] with the given per-hasher seed"] # [doc = " and [`SharedSeed`]."] # [inline (always)] pub fn with_seed (per_hasher_seed : u64 , shared_seed : & 'static SharedSeed) -> Self { Self { per_hasher_seed : per_hasher_seed ^ ARBITRARY3 , shared_seed , } } }
};
}
