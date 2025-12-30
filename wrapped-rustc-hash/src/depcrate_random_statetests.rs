// Generated macro for tests (module)
macro_rules! Depcrate_random_statetests {
() => {
// Module: crate::random_state
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: thread ; use crate :: FxHashMapRand ; # [test] fn cloned_random_states_are_equal () { let a = FxHashMapRand :: < & str , u32 > :: default () ; let b = a . clone () ; assert_eq ! (a . hasher () . seed , b . hasher () . seed) ; } # [test] fn random_states_are_different () { let a = FxHashMapRand :: < & str , u32 > :: default () ; let b = FxHashMapRand :: < & str , u32 > :: default () ; assert_ne ! (a . hasher () . seed , b . hasher () . seed) ; } # [test] fn random_states_are_different_cross_thread () { let a = FxHashMapRand :: < & str , u32 > :: default () ; let b = thread :: spawn (| | FxHashMapRand :: < & str , u32 > :: default ()) . join () . unwrap () ; assert_ne ! (a . hasher () . seed , b . hasher () . seed) ; } }
};
}
