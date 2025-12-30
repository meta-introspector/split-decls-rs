// Generated macro for seeded_rng (function)
macro_rules! Depcrate_join_testseeded_rng {
() => {
// Module: crate::join::test
// Provides: {"seeded_rng"}
// Dependencies: {}
fn seeded_rng () -> XorShiftRng { let mut seed = < XorShiftRng as SeedableRng > :: Seed :: default () ; (0 ..) . zip (seed . as_mut ()) . for_each (| (i , x) | * x = i) ; XorShiftRng :: from_seed (seed) }
};
}
