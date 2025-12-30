// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro128starstartests {
() => {
// Module: crate::xoshiro128starstar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoshiro128StarStar :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; let expected = [11520 , 0 , 5927040 , 70819200 , 2031721883 , 1637235492 , 1287239034 , 3734860849 , 3729100597 , 4258142804 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; } } # [test] fn test_jump () { let mut rng = Xoshiro128StarStar :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; rng . jump () ; assert_eq ! (rng . s [0] , 2843103750) ; assert_eq ! (rng . s [1] , 2038079848) ; assert_eq ! (rng . s [2] , 1533207345) ; assert_eq ! (rng . s [3] , 44816753) ; } # [test] fn test_long_jump () { let mut rng = Xoshiro128StarStar :: from_seed ([1 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 4 , 0 , 0 , 0]) ; rng . long_jump () ; assert_eq ! (rng . s [0] , 1611968294) ; assert_eq ! (rng . s [1] , 2125834322) ; assert_eq ! (rng . s [2] , 966769569) ; assert_eq ! (rng . s [3] , 3193880526) ; } }
};
}
