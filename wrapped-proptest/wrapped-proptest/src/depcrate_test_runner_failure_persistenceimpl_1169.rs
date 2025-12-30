// Generated macro for impl_1169 (impl)
macro_rules! Depcrate_test_runner_failure_persistenceimpl_1169 {
() => {
// Module: crate::test_runner::failure_persistence
// Provides: {"impl_1169"}
// Dependencies: {}
impl FromStr for PersistedSeed { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { Seed :: from_persistence (s) . map (PersistedSeed) . ok_or (()) } }
};
}
