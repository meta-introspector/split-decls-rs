// Generated macro for tests (module)
macro_rules! Depcrate_xoroshiro64starstartests {
() => {
// Module: crate::xoroshiro64starstar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoroshiro64StarStar :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0]) ; let expected = [3802928447 , 813792938 , 1618621494 , 2955957307 , 3252880261 , 1129983909 , 2539651700 , 1327610908 , 1757650787 , 2763843748 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; } } # [test] fn zero_seed () { let mut rng = Xoroshiro64StarStar :: seed_from_u64 (0) ; assert_ne ! (rng . next_u64 () , 0) ; } }
};
}
