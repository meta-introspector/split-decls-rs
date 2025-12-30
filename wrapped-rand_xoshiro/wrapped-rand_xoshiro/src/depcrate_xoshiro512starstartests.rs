// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro512starstartests {
() => {
// Module: crate::xoshiro512starstar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [rustfmt :: skip] fn reference () { let mut rng = Xoshiro512StarStar :: from_seed (Seed512 ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 5 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 7 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 8 , 0 , 0 , 0 , 0 , 0 , 0 , 0])) ; let expected = [11520 , 0 , 23040 , 23667840 , 144955163520 , 303992986974289920 , 25332796375735680 , 296904390158016 , 13911081092387501979 , 15304787717237593024 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
