// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro128plustests {
() => {
// Module: crate::xoshiro128plus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoshiro128Plus :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; let expected = [5 , 12295 , 25178119 , 27286542 , 39879690 , 1140358681 , 3276312097 , 4110231701 , 399823256 , 2144435200 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; } } # [test] fn test_jump () { let mut rng = Xoshiro128Plus :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; rng . jump () ; assert_eq ! (rng . s [0] , 2843103750) ; assert_eq ! (rng . s [1] , 2038079848) ; assert_eq ! (rng . s [2] , 1533207345) ; assert_eq ! (rng . s [3] , 44816753) ; } # [test] fn test_long_jump () { let mut rng = Xoshiro128Plus :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; rng . long_jump () ; assert_eq ! (rng . s [0] , 1611968294) ; assert_eq ! (rng . s [1] , 2125834322) ; assert_eq ! (rng . s [2] , 966769569) ; assert_eq ! (rng . s [3] , 3193880526) ; } }
};
}
