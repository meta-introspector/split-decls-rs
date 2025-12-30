// Generated macro for impl_12 (impl)
macro_rules! Depcrate_sfc32impl_12 {
() => {
// Module: crate::sfc32
// Provides: {"impl_12"}
// Dependencies: {}
impl SeedableRng for Sfc32 { type Seed = [u8 ; 12] ; # [doc = " Create a new `Sfc32`."] fn from_seed (seed : [u8 ; 12]) -> Sfc32 { let mut s = [0 ; 3] ; read_u32_into (& seed , & mut s) ; let mut rng = Sfc32 { a : s [0] , b : s [1] , c : s [2] , weyl : WEYL_INC , } ; for _ in 0 .. SEED_MIXING_STEPS { rng . next_u32 () ; } rng } }
};
}
