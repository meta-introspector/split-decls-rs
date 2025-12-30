// Generated macro for impl_153 (impl)
macro_rules! Depcrate_xoshiro512starstarimpl_153 {
() => {
// Module: crate::xoshiro512starstar
// Provides: {"impl_153"}
// Dependencies: {}
impl Xoshiro512StarStar { # [doc = " Jump forward, equivalently to 2^256 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^256 non-overlapping subsequences for"] # [doc = " parallel computations."] # [doc = ""] # [doc = " ```"] # [doc = " use rand_xoshiro::rand_core::SeedableRng;"] # [doc = " use rand_xoshiro::Xoshiro512StarStar;"] # [doc = ""] # [doc = " let rng1 = Xoshiro512StarStar::seed_from_u64(0);"] # [doc = " let mut rng2 = rng1.clone();"] # [doc = " rng2.jump();"] # [doc = " let mut rng3 = rng2.clone();"] # [doc = " rng3.jump();"] # [doc = " ```"] pub fn jump (& mut self) { impl_jump ! (u64 , self , [0x33ed89b6e7a353f9 , 0x760083d7955323be , 0x2837f2fbb5f22fae , 0x4b8c5674d309511c , 0xb11ac47a7ba28c25 , 0xf1be7667092bcc1c , 0x53851efdb6df0aaf , 0x1ebbc8b23eaf25db]) ; } # [doc = " Jump forward, equivalently to 2^384 calls to `next_u64()`."] # [doc = ""] # [doc = " This can be used to generate 2^128 starting points, from each of which"] # [doc = " `jump()` will generate 2^128 non-overlapping subsequences for parallel"] # [doc = " distributed computations."] pub fn long_jump (& mut self) { impl_jump ! (u64 , self , [0x11467fef8f921d28 , 0xa2a819f2e79c8ea8 , 0xa8299fc284b3959a , 0xb4d347340ca63ee1 , 0x1cb0940bedbff6ce , 0xd956c5c4fa1f8e17 , 0x915e38fd4eda93bc , 0x5b3ccdfa5d7daca5]) ; } }
};
}
