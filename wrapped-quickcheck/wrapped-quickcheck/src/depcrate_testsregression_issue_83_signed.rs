// Generated macro for regression_issue_83_signed (function)
macro_rules! Depcrate_testsregression_issue_83_signed {
() => {
// Module: crate::tests
// Provides: {"regression_issue_83_signed"}
// Dependencies: {}
# [test] fn regression_issue_83_signed () { fn prop (_ : i8) -> bool { true } QuickCheck :: new () . set_rng (Gen :: new (1024)) . quickcheck (prop as fn (i8) -> bool) ; }
};
}
