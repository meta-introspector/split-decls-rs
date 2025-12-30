// Generated macro for tests (module)
macro_rules! Depcrate_rngs_xoshiro128plusplustests {
() => {
// Module: crate::rngs::xoshiro128plusplus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Xoshiro128PlusPlus ; use rand_core :: { RngCore , SeedableRng } ; # [test] fn reference () { let mut rng = Xoshiro128PlusPlus :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; let expected = [641 , 1573767 , 3222811527 , 3517856514 , 836907274 , 4247214768 , 3867114732 , 1355841295 , 495546011 , 621204420 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; } } # [test] fn stable_seed_from_u64_and_from_seed () { let mut rng = Xoshiro128PlusPlus :: seed_from_u64 (0) ; let mut rng_from_seed_0 = Xoshiro128PlusPlus :: from_seed ([0 ; 16]) ; let expected = [1179900579 , 1938959192 , 3089844957 , 3657088315 , 1015453891 , 479942911 , 3433842246 , 669252886 , 3985671746 , 2737205563 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; assert_eq ! (rng_from_seed_0 . next_u32 () , e) ; } } }
};
}
