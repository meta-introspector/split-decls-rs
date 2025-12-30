// Generated macro for tests (module)
macro_rules! Depcrate_xoroshiro128starstartests {
() => {
// Module: crate::xoroshiro128starstar
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoroshiro128StarStar :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; let expected = [5760 , 97769243520 , 9706862127477703552 , 9223447511460779954 , 8358291023205304566 , 15695619998649302768 , 8517900938696309774 , 16586480348202605369 , 6959129367028440372 , 16822147227405758281 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
