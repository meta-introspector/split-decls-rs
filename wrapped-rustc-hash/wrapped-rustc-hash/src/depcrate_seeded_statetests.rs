// Generated macro for tests (module)
macro_rules! Depcrate_seeded_statetests {
() => {
// Module: crate::seeded_state
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use core :: hash :: BuildHasher ; use crate :: FxSeededState ; # [test] fn cloned_seeded_states_are_equal () { let seed = 2 ; let a = FxSeededState :: with_seed (seed) ; let b = a . clone () ; assert_eq ! (a . seed , b . seed) ; assert_eq ! (a . seed , seed) ; assert_eq ! (a . build_hasher () . hash , b . build_hasher () . hash) ; } # [test] fn same_seed_produces_same_hasher () { let seed = 1 ; let a = FxSeededState :: with_seed (seed) ; let b = FxSeededState :: with_seed (seed) ; assert_eq ! (a . build_hasher () . hash , b . build_hasher () . hash) ; } # [test] fn different_states_are_different () { let a = FxSeededState :: with_seed (1) ; let b = FxSeededState :: with_seed (2) ; assert_ne ! (a . build_hasher () . hash , b . build_hasher () . hash) ; } }
};
}
