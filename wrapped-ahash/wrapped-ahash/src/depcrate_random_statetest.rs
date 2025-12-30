// Generated macro for test (module)
macro_rules! Depcrate_random_statetest {
() => {
// Module: crate::random_state
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_unique () { let a = RandomState :: generate_with (1 , 2 , 3 , 4) ; let b = RandomState :: generate_with (1 , 2 , 3 , 4) ; assert_ne ! (a . build_hasher () . finish () , b . build_hasher () . finish ()) ; } # [cfg (all (feature = "runtime-rng" , not (all (feature = "compile-time-rng" , test))))] # [test] fn test_not_pi () { assert_ne ! (PI_U64X4 , get_fixed_seeds () [0]) ; } # [cfg (all (feature = "compile-time-rng" , any (not (feature = "runtime-rng") , test)))] # [test] fn test_not_pi_const () { assert_ne ! (PI_U64X4 , get_fixed_seeds () [0]) ; } # [cfg (all (not (feature = "runtime-rng") , not (feature = "compile-time-rng")))] # [test] fn test_pi () { assert_eq ! (PI_U64X4 , get_fixed_seeds () [0]) ; } # [test] fn test_with_seeds_const () { const _CONST_RANDOM_STATE : RandomState = RandomState :: with_seeds (17 , 19 , 21 , 23) ; } }
};
}
