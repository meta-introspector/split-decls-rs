// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_test_runner_rngimpl_1224 {
() => {
// Module: crate::test_runner::rng
// Provides: {"impl_1224"}
// Dependencies: {}
impl str :: FromStr for RngAlgorithm { type Err = () ; fn from_str (s : & str) -> Result < Self , () > { RngAlgorithm :: from_persistence_key (s) . ok_or (()) } }
};
}
