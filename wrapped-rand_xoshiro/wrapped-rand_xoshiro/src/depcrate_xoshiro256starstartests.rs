// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro256starstartests {
() => {
// Module: crate::xoshiro256starstar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoshiro256StarStar :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,]) ; let expected = [11520 , 0 , 1509978240 , 1215971899390074240 , 1216172134540287360 , 607988272756665600 , 16172922978634559625 , 8476171486693032832 , 10595114339597558777 , 2904607092377533576 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
