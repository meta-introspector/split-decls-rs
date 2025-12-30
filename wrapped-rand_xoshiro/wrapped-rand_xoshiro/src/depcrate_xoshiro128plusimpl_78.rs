// Generated macro for impl_78 (impl)
macro_rules! Depcrate_xoshiro128plusimpl_78 {
() => {
// Module: crate::xoshiro128plus
// Provides: {"impl_78"}
// Dependencies: {}
impl Xoshiro128Plus { # [doc = " Jump forward, equivalently to 2^64 calls to `next_u32()`."] # [doc = ""] # [doc = " This can be used to generate 2^64 non-overlapping subsequences for"] # [doc = " parallel computations."] # [doc = ""] # [doc = " ```"] # [doc = " use rand_xoshiro::rand_core::SeedableRng;"] # [doc = " use rand_xoshiro::Xoroshiro128StarStar;"] # [doc = ""] # [doc = " let rng1 = Xoroshiro128StarStar::seed_from_u64(0);"] # [doc = " let mut rng2 = rng1.clone();"] # [doc = " rng2.jump();"] # [doc = " let mut rng3 = rng2.clone();"] # [doc = " rng3.jump();"] # [doc = " ```"] pub fn jump (& mut self) { impl_jump ! (u32 , self , [0x8764000b , 0xf542d2d3 , 0x6fa035c3 , 0x77f2db5b]) ; } # [doc = " Jump forward, equivalently to 2^96 calls to `next_u32()`."] # [doc = ""] # [doc = " This can be used to generate 2^32 starting points, from each of which"] # [doc = " `jump()` will generate 2^32 non-overlapping subsequences for parallel"] # [doc = " distributed computations."] pub fn long_jump (& mut self) { impl_jump ! (u32 , self , [0xb523952e , 0x0b6f099f , 0xccf5a0ef , 0x1c580662]) ; } }
};
}
