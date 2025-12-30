// Generated macro for impl_13 (impl)
macro_rules! Depcrate_isaacimpl_13 {
() => {
// Module: crate::isaac
// Provides: {"impl_13"}
// Dependencies: {}
impl SeedableRng for IsaacRng { type Seed = < IsaacCore as SeedableRng > :: Seed ; # [inline] fn from_seed (seed : Self :: Seed) -> Self { IsaacRng (BlockRng :: < IsaacCore > :: from_seed (seed)) } # [doc = " Create an ISAAC random number generator using an `u64` as seed."] # [doc = " If `seed == 0` this will produce the same stream of random numbers as"] # [doc = " the reference implementation when used unseeded."] # [inline] fn seed_from_u64 (seed : u64) -> Self { IsaacRng (BlockRng :: < IsaacCore > :: seed_from_u64 (seed)) } # [inline] fn from_rng < R > (rng : & mut R) -> Self where R : RngCore + ? Sized , { IsaacRng (BlockRng :: < IsaacCore > :: from_rng (rng)) } # [inline] fn try_from_rng < S > (rng : & mut S) -> Result < Self , S :: Error > where S : TryRngCore + ? Sized , { BlockRng :: < IsaacCore > :: try_from_rng (rng) . map (IsaacRng) } }
};
}
