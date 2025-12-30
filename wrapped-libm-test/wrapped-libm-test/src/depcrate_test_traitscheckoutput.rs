// Generated macro for CheckOutput (trait)
macro_rules! Depcrate_test_traitsCheckOutput {
() => {
// Module: crate::test_traits
// Provides: {"CheckOutput"}
// Dependencies: {}
# [doc = " A trait to implement on any output type so we can verify it in a generic way."] pub trait CheckOutput < Input > : Sized { # [doc = " Validate `self` (actual) and `expected` are the same."] # [doc = ""] # [doc = " `input` is only used here for error messages."] fn validate (self , expected : Self , input : Input , ctx : & CheckCtx) -> TestResult ; }
};
}
