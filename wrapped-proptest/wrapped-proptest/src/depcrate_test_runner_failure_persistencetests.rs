// Generated macro for tests (module)
macro_rules! Depcrate_test_runner_failure_persistencetests {
() => {
// Module: crate::test_runner::failure_persistence
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: PersistedSeed ; use crate :: test_runner :: rng :: Seed ; pub const INC_SEED : PersistedSeed = PersistedSeed (Seed :: XorShift ([0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,])) ; pub const HI_PATH : Option < & str > = Some ("hi") ; pub const UNREL_PATH : Option < & str > = Some ("unrelated") ; }
};
}
