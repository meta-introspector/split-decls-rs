// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro512plustests {
() => {
// Module: crate::xoshiro512plus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { # [rustfmt :: skip] let mut rng = Xoshiro512Plus :: from_seed (Seed512 ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 5 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 7 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 8 , 0 , 0 , 0 , 0 , 0 , 0 , 0])) ; let expected = [4 , 8 , 4113 , 25169936 , 52776585412635 , 57174648719367 , 9223482039571869716 , 9331471677901559830 , 9340533895746033672 , 14078399799840753678 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
