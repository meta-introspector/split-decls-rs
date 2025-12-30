// Generated macro for RngSeed (enum)
macro_rules! Depcrate_test_runner_configRngSeed {
() => {
// Module: crate::test_runner::config
// Provides: {"RngSeed"}
// Dependencies: {}
# [doc = " The seed for the RNG, can either be random or specified as a u64."] # [derive (Debug , Clone , Copy , PartialEq)] pub enum RngSeed { # [doc = " Default case, use a random value"] Random , # [doc = " Use a specific value to generate a seed"] Fixed (u64) }
};
}
