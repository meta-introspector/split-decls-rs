// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro256plusplustests {
() => {
// Module: crate::xoshiro256plusplus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoshiro256PlusPlus :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,]) ; let expected = [41943041 , 58720359 , 3588806011781223 , 3591011842654386 , 9228616714210784205 , 9973669472204895162 , 14011001112246962877 , 12406186145184390807 , 15849039046786891736 , 10450023813501588000 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
