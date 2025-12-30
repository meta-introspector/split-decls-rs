// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro512plusplustests {
() => {
// Module: crate::xoshiro512plusplus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoshiro512PlusPlus :: from_seed (Seed512 ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 5 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 6 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 7 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 8 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,])) ; let expected = [524291 , 1048578 , 539099140 , 3299073855497 , 6917532603230064654 , 7494048333530275843 , 14418333309547923463 , 10960079161595355914 , 18279570946505382726 , 10209173166699159237 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
