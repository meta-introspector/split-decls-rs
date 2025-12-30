// Generated macro for impl_123 (impl)
macro_rules! Depcrate_xoshiro256starstarimpl_123 {
() => {
// Module: crate::xoshiro256starstar
// Provides: {"impl_123"}
// Dependencies: {}
impl Xoshiro256StarStar { # [doc = " Jump forward, equivalently to 2^128 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^128 non-overlapping subsequences for"] # [doc = " parallel computations."] # [doc = ""] # [doc = " ```"] # [doc = " use rand_xoshiro::rand_core::SeedableRng;"] # [doc = " use rand_xoshiro::Xoshiro256StarStar;"] # [doc = ""] # [doc = " let rng1 = Xoshiro256StarStar::seed_from_u64(0);"] # [doc = " let mut rng2 = rng1.clone();"] # [doc = " rng2.jump();"] # [doc = " let mut rng3 = rng2.clone();"] # [doc = " rng3.jump();"] # [doc = " ```"] pub fn jump (& mut self) { impl_jump ! (u64 , self , [0x180ec6d33cfd0aba , 0xd5a61266f0c9392c , 0xa9582618e03fc9aa , 0x39abdc4529b1661c]) ; } # [doc = " Jump forward, equivalently to 2^192 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^64 starting points, from each of which"] # [doc = " `jump()` will generate 2^64 non-overlapping subsequences for parallel"] # [doc = " distributed computations."] pub fn long_jump (& mut self) { impl_jump ! (u64 , self , [0x76e15d3efefdcbbf , 0xc5004e441c522fb3 , 0x77710069854ee241 , 0x39109bb02acbe635]) ; } }
};
}
