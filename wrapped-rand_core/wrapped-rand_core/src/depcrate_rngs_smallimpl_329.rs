// Generated macro for impl_329 (impl)
macro_rules! Depcrate_rngs_smallimpl_329 {
() => {
// Module: crate::rngs::small
// Provides: {"impl_329"}
// Dependencies: {}
impl SeedableRng for SmallRng { type Seed = [u8 ; 32] ; # [inline (always)] fn from_seed (seed : Self :: Seed) -> Self { const LEN : usize = core :: mem :: size_of :: < < Rng as SeedableRng > :: Seed > () ; let seed = (& seed [.. LEN]) . try_into () . unwrap () ; SmallRng (Rng :: from_seed (seed)) } # [inline (always)] fn seed_from_u64 (state : u64) -> Self { SmallRng (Rng :: seed_from_u64 (state)) } }
};
}
