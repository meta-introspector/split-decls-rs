// Generated macro for tests (module)
macro_rules! Depcrate_xoshiro256plustests {
() => {
// Module: crate::xoshiro256plus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoshiro256Plus :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 3 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 4 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,]) ; let expected = [5 , 211106232532999 , 211106635186183 , 9223759065350669058 , 9250833439874351877 , 13862484359527728515 , 2346507365006083650 , 1168864526675804870 , 34095955243042024 , 3466914240207415127 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
