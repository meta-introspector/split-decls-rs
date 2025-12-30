// Generated macro for impl_34 (impl)
macro_rules! Depcrate_isaac64impl_34 {
() => {
// Module: crate::isaac64
// Provides: {"impl_34"}
// Dependencies: {}
impl SeedableRng for Isaac64Rng { type Seed = < Isaac64Core as SeedableRng > :: Seed ; # [inline] fn from_seed (seed : Self :: Seed) -> Self { Isaac64Rng (BlockRng64 :: < Isaac64Core > :: from_seed (seed)) } # [doc = " Create an ISAAC random number generator using an `u64` as seed."] # [doc = " If `seed == 0` this will produce the same stream of random numbers as"] # [doc = " the reference implementation when used unseeded."] # [inline] fn seed_from_u64 (seed : u64) -> Self { Isaac64Rng (BlockRng64 :: < Isaac64Core > :: seed_from_u64 (seed)) } # [inline] fn from_rng < R > (rng : & mut R) -> Self where R : RngCore + ? Sized , { Isaac64Rng (BlockRng64 :: < Isaac64Core > :: from_rng (rng)) } # [inline] fn try_from_rng < S > (rng : & mut S) -> Result < Self , S :: Error > where S : TryRngCore + ? Sized , { BlockRng64 :: < Isaac64Core > :: try_from_rng (rng) . map (Isaac64Rng) } }
};
}
