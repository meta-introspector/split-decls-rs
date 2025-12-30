// Generated macro for max (function)
macro_rules! Depcrate_testsmax {
() => {
// Module: crate::tests
// Provides: {"max"}
// Dependencies: {}
# [test] fn max () { fn prop (x : isize , y : isize) -> TestResult { if x > y { TestResult :: discard () } else { TestResult :: from_bool (:: std :: cmp :: max (x , y) == y) } } quickcheck (prop as fn (isize , isize) -> TestResult) ; }
};
}
