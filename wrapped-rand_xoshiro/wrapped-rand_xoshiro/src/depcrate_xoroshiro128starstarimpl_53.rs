// Generated macro for impl_53 (impl)
macro_rules! Depcrate_xoroshiro128starstarimpl_53 {
() => {
// Module: crate::xoroshiro128starstar
// Provides: {"impl_53"}
// Dependencies: {}
impl Xoroshiro128StarStar { # [doc = " Jump forward, equivalently to 2^64 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^64 non-overlapping subsequences for"] # [doc = " parallel computations."] # [doc = ""] # [doc = " ```"] # [doc = " use rand_xoshiro::rand_core::SeedableRng;"] # [doc = " use rand_xoshiro::Xoroshiro128StarStar;"] # [doc = ""] # [doc = " let rng1 = Xoroshiro128StarStar::seed_from_u64(0);"] # [doc = " let mut rng2 = rng1.clone();"] # [doc = " rng2.jump();"] # [doc = " let mut rng3 = rng2.clone();"] # [doc = " rng3.jump();"] # [doc = " ```"] pub fn jump (& mut self) { impl_jump ! (u64 , self , [0xdf900294d8f554a5 , 0x170865df4b3201fc]) ; } # [doc = " Jump forward, equivalently to 2^96 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^32 starting points, from each of which"] # [doc = " `jump()` will generate 2^32 non-overlapping subsequences for parallel"] # [doc = " distributed computations."] pub fn long_jump (& mut self) { impl_jump ! (u64 , self , [0xd2a98b26625eee7b , 0xdddf9b1090aa7ac1]) ; } }
};
}
