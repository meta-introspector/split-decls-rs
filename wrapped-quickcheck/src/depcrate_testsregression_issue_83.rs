// Generated macro for regression_issue_83 (function)
macro_rules! Depcrate_testsregression_issue_83 {
() => {
// Module: crate::tests
// Provides: {"regression_issue_83"}
// Dependencies: {}
# [test] fn regression_issue_83 () { fn prop (_ : u8) -> bool { true } QuickCheck :: new () . set_rng (Gen :: new (1024)) . quickcheck (prop as fn (u8) -> bool) ; }
};
}
