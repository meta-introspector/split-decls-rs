// Generated macro for TestOutcome (trait)
macro_rules! DepcrateTestOutcome {
() => {
// Module: crate
// Provides: {"TestOutcome"}
// Dependencies: {}
# [doc = " Indicates whether a test succeeded or failed."] # [doc = ""] # [doc = " This is comparable to the `Termination` trait in libstd, except stable and tailored towards the"] # [doc = " needs of defmt-test. It is implemented for `()`, which always indicates success, and `Result`,"] # [doc = " where `Ok` indicates success."] pub trait TestOutcome : Format + sealed :: Sealed { fn is_success (& self) -> bool ; }
};
}
