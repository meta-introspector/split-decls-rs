// Generated macro for TestRunner (enum)
macro_rules! Depcrate_envTestRunner {
() => {
// Module: crate::env
// Provides: {"TestRunner"}
// Dependencies: {}
# [doc = " The test runner to use."] # [cfg (feature = "_cargo_insta_internal")] # [derive (Clone , Copy , Debug , PartialEq , Eq , clap :: ValueEnum)] pub enum TestRunner { Auto , CargoTest , Nextest , }
};
}
