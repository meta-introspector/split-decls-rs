// Generated macro for tests (module)
macro_rules! Depcrate_xoroshiro128plusplustests {
() => {
// Module: crate::xoroshiro128plusplus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoroshiro128PlusPlus :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; let expected = [393217 , 669327710093319 , 1732421326133921491 , 11394790081659126983 , 9555452776773192676 , 3586421180005889563 , 1691397964866707553 , 10735626796753111697 , 15216282715349408991 , 14247243556711267923 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
