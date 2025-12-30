// Generated macro for seeded_rng (function)
macro_rules! Depcrate_engine_testsseeded_rng {
() => {
// Module: crate::engine::tests
// Provides: {"seeded_rng"}
// Dependencies: {}
fn seeded_rng () -> impl rand :: Rng { rngs :: SmallRng :: from_entropy () }
};
}
