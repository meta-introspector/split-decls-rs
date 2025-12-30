// Generated macro for regression_issue_107_hang (function)
macro_rules! Depcrate_testsregression_issue_107_hang {
() => {
// Module: crate::tests
// Provides: {"regression_issue_107_hang"}
// Dependencies: {}
# [test] # [should_panic] fn regression_issue_107_hang () { fn prop (a : Vec < u8 >) -> bool { a . contains (& 1) } quickcheck (prop as fn (_) -> bool) ; }
};
}
