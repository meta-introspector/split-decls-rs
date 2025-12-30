// Generated macro for impl_44 (impl)
macro_rules! Depcrate_xoroshiro128plusplusimpl_44 {
() => {
// Module: crate::xoroshiro128plusplus
// Provides: {"impl_44"}
// Dependencies: {}
impl Xoroshiro128PlusPlus { # [doc = " Jump forward, equivalently to 2^64 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^64 non-overlapping subsequences for"] # [doc = " parallel computations."] # [doc = ""] # [doc = " ```"] # [doc = " use rand_xoshiro::rand_core::SeedableRng;"] # [doc = " use rand_xoshiro::Xoroshiro128PlusPlus;"] # [doc = ""] # [doc = " let rng1 = Xoroshiro128PlusPlus::seed_from_u64(0);"] # [doc = " let mut rng2 = rng1.clone();"] # [doc = " rng2.jump();"] # [doc = " let mut rng3 = rng2.clone();"] # [doc = " rng3.jump();"] # [doc = " ```"] pub fn jump (& mut self) { impl_jump ! (u64 , self , [0x2bd7a6a6e99c2ddc , 0x0992ccaf6a6fca05]) ; } # [doc = " Jump forward, equivalently to 2^96 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^32 starting points, from each of which"] # [doc = " `jump()` will generate 2^32 non-overlapping subsequences for parallel"] # [doc = " distributed computations."] pub fn long_jump (& mut self) { impl_jump ! (u64 , self , [0x360fd5f2cf8d5d99 , 0x9c6e6877736c46e3]) ; } }
};
}
