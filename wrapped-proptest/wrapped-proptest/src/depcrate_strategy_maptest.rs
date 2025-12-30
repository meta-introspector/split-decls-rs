// Generated macro for test (module)
macro_rules! Depcrate_strategy_maptest {
() => {
// Module: crate::strategy::map
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use std :: collections :: HashSet ; use rand :: RngCore ; use super :: * ; use crate :: strategy :: just :: Just ; # [test] fn test_map () { TestRunner :: default () . run (& (0 .. 10) . prop_map (| v | v * 2) , | v | { assert ! (0 == v % 2) ; Ok (()) }) . unwrap () ; } # [test] fn test_map_into () { TestRunner :: default () . run (& (0 .. 10u8) . prop_map_into :: < usize > () , | v | { assert ! (v < 10) ; Ok (()) }) . unwrap () ; } # [test] fn perturb_uses_same_rng_every_time () { let mut runner = TestRunner :: default () ; let input = Just (1) . prop_perturb (| v , mut rng | v + rng . next_u32 ()) ; for _ in 0 .. 16 { let value = input . new_tree (& mut runner) . unwrap () ; assert_eq ! (value . current () , value . current ()) ; } } # [test] fn perturb_uses_varying_random_seeds () { let mut runner = TestRunner :: default () ; let input = Just (1) . prop_perturb (| v , mut rng | v + rng . next_u32 ()) ; let mut seen = HashSet :: new () ; for _ in 0 .. 64 { seen . insert (input . new_tree (& mut runner) . unwrap () . current ()) ; } assert_eq ! (64 , seen . len ()) ; } }
};
}
