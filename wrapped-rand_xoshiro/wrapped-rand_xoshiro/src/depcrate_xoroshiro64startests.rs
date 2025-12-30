// Generated macro for tests (module)
macro_rules! Depcrate_xoroshiro64startests {
() => {
// Module: crate::xoroshiro64star
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoroshiro64Star :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0]) ; let expected = [2654435771 , 327208753 , 4063491769 , 4259754937 , 261922412 , 168123673 , 552743735 , 1672597395 , 1031040050 , 2755315674 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; } } # [test] fn zero_seed () { let mut rng = Xoroshiro64Star :: seed_from_u64 (0) ; assert_ne ! (rng . next_u64 () , 0) ; } }
};
}
