// Generated macro for sort (function)
macro_rules! Depcrate_testssort {
() => {
// Module: crate::tests
// Provides: {"sort"}
// Dependencies: {}
# [test] fn sort () { fn prop (mut xs : Vec < isize >) -> bool { xs . sort_unstable () ; for i in xs . windows (2) { if i [0] > i [1] { return false ; } } true } quickcheck (prop as fn (Vec < isize >) -> bool) ; }
};
}
