// Generated macro for impl_25 (impl)
macro_rules! Depcrate_sfc64impl_25 {
() => {
// Module: crate::sfc64
// Provides: {"impl_25"}
// Dependencies: {}
impl SeedableRng for Sfc64 { type Seed = [u8 ; 24] ; # [doc = " Create a new `Sfc64`."] fn from_seed (seed : [u8 ; 24]) -> Sfc64 { let mut s = [0 ; 3] ; read_u64_into (& seed , & mut s) ; let mut rng = Sfc64 { a : s [0] , b : s [1] , c : s [2] , weyl : WEYL_INC , } ; for _ in 0 .. SEED_MIXING_STEPS { rng . next_u64 () ; } rng } }
};
}
