// Generated macro for tests (module)
macro_rules! Depcrate_xoroshiro128plustests {
() => {
// Module: crate::xoroshiro128plus
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn reference () { let mut rng = Xoroshiro128Plus :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; let expected = [3 , 412333834243 , 2360170716294286339 , 9295852285959843169 , 2797080929874688578 , 6019711933173041966 , 3076529664176959358 , 3521761819100106140 , 7493067640054542992 , 920801338098114767 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
